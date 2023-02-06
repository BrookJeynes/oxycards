pub mod models;
pub mod ui;

use std::io::stdout;
use std::path::Path;
use std::{error::Error, fs, io};

use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use models::card::Card;
use models::card_types::fill_in_the_blanks::FillInTheBlanks;
use models::card_types::flashcard::FlashCard;
use models::card_types::multiple_answer::MultipleAnswer;
use models::card_types::multiple_choice::MultipleChoice;
use models::card_types::order::Order;
use models::stateful_list::StatefulList;
use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;
use ui::ui;

#[derive(Debug)]
enum ParsingError {
    NoCardType,
}

pub struct AppState {
    pub cards: StatefulList<Card>,
    pub incorrect_answers: usize,
    pub correct_answers: usize,
}

impl AppState {
    fn new(cards: Vec<Card>) -> Self {
        Self {
            cards: StatefulList::with_items(cards),
            incorrect_answers: 0,
            correct_answers: 0,
        }
    }
}

fn extract_card_title(content: &String) -> (String, String) {
    // Don't unwrap
    let question = content.lines().nth(0).unwrap()[1..].trim().to_string();
    let content = content.lines().skip(1).collect::<Vec<&str>>().join("\n");

    (question, content)
}

fn card_parser(content: String) -> Result<Vec<Card>, ParsingError> {
    let cards: Vec<Card> = content
        .split("---")
        .map(|section| {
            let sections = section
                .trim()
                .split("\n\n")
                .filter(|item| !item.is_empty())
                .collect::<Vec<&str>>();

            match sections[0].to_lowercase().as_str() {
                "flashcard" => Card::FlashCard(FlashCard::parse_raw(sections[1].to_string())),
                "multiple_choice" => {
                    Card::MultipleChoice(MultipleChoice::parse_raw(sections[1].to_string()))
                }
                "multiple_answer" => {
                    Card::MultipleAnswer(MultipleAnswer::parse_raw(sections[1].to_string()))
                }
                "fill_in_the_blanks" => {
                    Card::FillInTheBlanks(FillInTheBlanks::parse_raw(sections[1].to_string()))
                }
                "order" => Card::Order(Order::parse_raw(sections[1].to_string())),
                // Replace with ParsingError datatype
                _ => panic!("Parsing Error"),
            }
        })
        .collect();

    Ok(cards)
}

fn read_from_file(path: &Path) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = read_from_file(Path::new("input.md"))?;
    // Todo: Don't unwrap()
    let cards = card_parser(content).unwrap();

    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app_state = AppState::new(cards);
    let res = run_app(&mut terminal, app_state);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app_state: AppState,
) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|f| ui(f, &mut app_state))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                // Card navigation keys
                KeyCode::Char('h') | KeyCode::Left => app_state.cards.previous(),
                KeyCode::Char('l') | KeyCode::Right => app_state.cards.next(),

                KeyCode::Char(' ') => {
                    if let Some(val) = app_state.cards.selected_value() {
                        match val {
                            Card::FlashCard(card) => card.flip_card(),
                            Card::MultipleAnswer(card) => {
                                if let Some(index) = card.choices.selected() {
                                    card.choices.items[index].select()
                                }
                            }
                            Card::MultipleChoice(card) => {
                                if let Some(index) = card.choices.selected() {
                                    if let None = card.correct_answer {
                                        card.unselect_all();

                                        card.choices.items[index].select()
                                    }
                                }
                            }
                            Card::Order(card) => {
                                if let Some(index) = card.shuffled.selected() {
                                    card.shuffled.items[index].select()
                                }

                                if let Some((a, b)) = card.multiple_selected() {
                                    card.shuffled.swap(a, b);
                                    card.unselect_all();
                                }
                            }
                            _ => {}
                        }
                    }
                }
                KeyCode::Char('k') | KeyCode::Up => {
                    if let Some(val) = app_state.cards.selected_value() {
                        match val {
                            Card::MultipleChoice(card) => card.choices.previous(),
                            Card::MultipleAnswer(card) => card.choices.previous(),
                            Card::Order(card) => card.shuffled.previous(),
                            _ => {}
                        }
                    }
                }
                KeyCode::Char('j') | KeyCode::Down => {
                    if let Some(val) = app_state.cards.selected_value() {
                        match val {
                            Card::MultipleChoice(card) => card.choices.next(),
                            Card::MultipleAnswer(card) => card.choices.next(),
                            Card::Order(card) => card.shuffled.next(),
                            _ => {}
                        }
                    }
                }
                KeyCode::Enter => {
                    if let Some(val) = app_state.cards.selected_value() {
                        match val {
                            Card::MultipleChoice(card) => {
                                if let Some(value) = card.validate_answer() {
                                    if value {
                                        app_state.correct_answers += 1;
                                    } else {
                                        app_state.incorrect_answers += 1;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }

                // Exit keys
                KeyCode::Char('q') => return Ok(()),

                _ => {}
            }
        }
    }
}
