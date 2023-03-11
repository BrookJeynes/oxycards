pub mod models;
pub mod ui;

use clap::Parser;
use models::args::Args;
use models::errors::errors::Errors;

use std::path::Path;
use std::{error::Error, fs, io};

use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};

use models::card::Card;
use models::stateful_list::StatefulList;
use models::user_answer::UserAnswer;

use tui::backend::{Backend, CrosstermBackend};
use tui::Terminal;
use ui::ui;

pub enum InputMode {
    Normal,
    Editing,
}

pub struct Score {
    incorrect: usize,
    correct: usize,
}

impl Score {
    fn add_incorrect(&mut self) {
        self.incorrect += 1;
    }

    fn add_correct(&mut self) {
        self.correct += 1;
    }
}

impl Default for Score {
    fn default() -> Self {
        Self {
            incorrect: 0,
            correct: 0,
        }
    }
}

pub struct AppState {
    pub cards: StatefulList<Card>,
    pub input_mode: InputMode,
    pub score: Score,
}

impl AppState {
    fn new(cards: Vec<Card>) -> Self {
        Self {
            cards: StatefulList::with_items(cards),
            score: Score::default(),
            input_mode: InputMode::Normal,
        }
    }
}

fn read_from_file(path: &Path) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let path = Path::new(&args.path);

    if let Err(err) = Args::validate_file(path) {
        Errors::throw_file_error(err)
    };

    let content = read_from_file(path)?;
    let cards = match Card::card_parser(content) {
        Ok(cards) => cards,
        Err(err) => Errors::throw_parsing_error(err),
    };

    let mut terminal = init_terminal()?;

    let app_state = AppState::new(cards);
    let res = run_app(&mut terminal, app_state);

    reset_terminal()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

/// Initializes the terminal.
fn init_terminal() -> Result<Terminal<CrosstermBackend<io::Stdout>>, Box<dyn Error>> {
    execute!(io::stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(io::stdout());

    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // Because a panic interrupts the normal control flow, manually resetting the
    // terminal at the end of `main` won't do us any good. Instead, we need to
    // make sure to set up a panic hook that first resets the terminal before
    // handling the panic. This both reuses the standard panic hook to ensure a
    // consistent panic handling UX and properly resets the terminal to not
    // distort the output.
    let original_hook = std::panic::take_hook();

    std::panic::set_hook(Box::new(move |panic| {
        reset_terminal().unwrap();
        original_hook(panic);
    }));

    Ok(terminal)
}

/// Resets the terminal.
fn reset_terminal() -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    crossterm::execute!(io::stdout(), LeaveAlternateScreen)?;

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app_state: AppState,
) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|f| ui(f, &mut app_state))?;

        if let Some(val) = app_state.cards.selected_value() {
            match val {
                Card::FillInTheBlanks(card) => {
                    if let UserAnswer::Undecided = card.user_answer {
                        app_state.input_mode = InputMode::Editing
                    }
                }
                _ => app_state.input_mode = InputMode::Normal,
            }
        }

        if let Event::Key(key) = event::read()? {
            match app_state.input_mode {
                InputMode::Normal => match key.code {
                    // Card navigation keys
                    KeyCode::Char('h') | KeyCode::Left => app_state.cards.previous(),
                    KeyCode::Char('l') | KeyCode::Right => app_state.cards.next(),

                    KeyCode::Char(' ') => {
                        if let Some(val) = app_state.cards.selected_value() {
                            match val {
                                Card::FlashCard(card) => card.show_back(),
                                Card::MultipleAnswer(card) => {
                                    if let UserAnswer::Undecided = card.user_answer {
                                        if let Some(index) = card.choices.selected() {
                                            card.choices.items[index].select()
                                        }
                                    }
                                }
                                Card::MultipleChoice(card) => {
                                    if let Some(index) = card.choices.selected() {
                                        if let UserAnswer::Undecided = card.user_answer {
                                            card.unselect_all();

                                            card.choices.items[index].select()
                                        }
                                    }
                                }
                                Card::Order(card) => {
                                    if let UserAnswer::Undecided = card.user_answer {
                                        if let Some(index) = card.shuffled.selected() {
                                            card.shuffled.items[index].select()
                                        }

                                        if let Some((a, b)) = card.multiple_selected() {
                                            card.shuffled.swap(a, b);
                                            card.unselect_all();
                                        }
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
                        if let Some(card) = app_state.cards.selected_value() {
                            if !card.check_answered() {
                                match card.validate_answer() {
                                    UserAnswer::Correct => app_state.score.add_correct(),
                                    UserAnswer::Incorrect => app_state.score.add_incorrect(),
                                    UserAnswer::Undecided => {}
                                }
                            }
                        }
                    }
                    KeyCode::Char('y') => {
                        if let Some(val) = app_state.cards.selected_value() {
                            match val {
                                Card::FlashCard(card) => {
                                    if card.show_validation_popup
                                        && card.user_answer == UserAnswer::Undecided
                                    {
                                        card.user_answer = UserAnswer::Correct;
                                        app_state.score.add_correct()
                                    }
                                }

                                _ => {}
                            }
                        }
                    }
                    KeyCode::Char('n') => {
                        if let Some(val) = app_state.cards.selected_value() {
                            match val {
                                Card::FlashCard(card) => {
                                    if card.show_validation_popup
                                        && card.user_answer == UserAnswer::Undecided
                                    {
                                        card.user_answer = UserAnswer::Incorrect;
                                        app_state.score.add_incorrect()
                                    }
                                }

                                _ => {}
                            }
                        }
                    }

                    // Exit keys
                    KeyCode::Char('q') => return Ok(()),

                    _ => {}
                },
                InputMode::Editing => match key.code {
                    KeyCode::Tab => {
                        if let Some(val) = app_state.cards.selected_value() {
                            match val {
                                Card::FillInTheBlanks(card) => {
                                    card.next();
                                }

                                _ => {}
                            }
                        }
                    }
                    KeyCode::Enter => {
                        if let Some(card) = app_state.cards.selected_value() {
                            if !card.check_answered() {
                                match card.validate_answer() {
                                    UserAnswer::Correct => app_state.score.add_correct(),
                                    UserAnswer::Incorrect => app_state.score.add_incorrect(),
                                    UserAnswer::Undecided => {}
                                }

                                app_state.input_mode = InputMode::Normal;
                            }
                        }
                    }
                    KeyCode::Char(c) => {
                        if let Some(card_) = app_state.cards.selected_value() {
                            if let Card::FillInTheBlanks(card) = card_ {
                                card.user_input[card.blank_index].push(c);
                                card.update_output();
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        if let Some(card_) = app_state.cards.selected_value() {
                            if let Card::FillInTheBlanks(card) = card_ {
                                card.user_input[card.blank_index].pop();
                                card.update_output();
                            }
                        }
                    }
                    KeyCode::Left => app_state.cards.previous(),
                    KeyCode::Right => app_state.cards.next(),
                    // Exit keys
                    KeyCode::Esc => return Ok(()),
                    _ => {}
                },
            }
        }
    }
}
