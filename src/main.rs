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
use models::fill_in_the_blanks::FillInTheBlanks;
use models::flashcard::FlashCard;
use models::multiple_answer::MultipleAnswer;
use models::multiple_choice::MultipleChoice;
use models::order::Order;
use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;
use ui::ui;

#[derive(Debug)]
enum ParsingError {
    NoCardType,
}

pub struct Cards {
    pub current_card: usize,
    pub cards: Vec<Card>,
}

impl Cards {
    pub fn with_cards(cards: Vec<Card>) -> Self {
        Self {
            current_card: 0,
            cards,
        }
    }

    pub fn next(&mut self) {
        if self.current_card < self.cards.len() - 1 {
            self.current_card += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.current_card > 0 {
            self.current_card -= 1;
        }
    }

    pub fn selected(&self) -> &Card {
        self.cards
            .get(self.current_card)
            .expect("Will always return a valid card")
    }
}

pub struct AppState {
    pub cards: Cards,
}

impl AppState {
    fn new(cards: Vec<Card>) -> Self {
        Self {
            cards: Cards::with_cards(cards),
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

                // Exit keys
                KeyCode::Char('q') => return Ok(()),

                _ => {}
            }
        }
    }
}
