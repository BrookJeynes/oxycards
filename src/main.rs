pub mod models;
pub mod ui;

use clap::Parser;

use core::fmt;
use std::ffi::OsStr;
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

#[derive(PartialEq)]
enum FileType {
    MD,
}

impl FileType {
    fn from_str(file_extension: &str) -> Option<Self> {
        match file_extension {
            "md" => Some(FileType::MD),
            _ => None,
        }
    }
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileType::MD => write!(f, "md"),
        }
    }
}

enum FileError {
    InvalidFileType,
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileError::InvalidFileType => write!(f, "Invalid file type"),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Path to a quiz md file
    #[arg(short, long)]
    path: String,
}

impl Args {
    fn validate_file(file: &Path) -> Result<(), FileError> {
        match file.extension().and_then(OsStr::to_str) {
            Some(extension) => {
                if let Some(_) = FileType::from_str(extension) {
                    return Ok(());
                }
            }
            None => return Err(FileError::InvalidFileType),
        }

        Err(FileError::InvalidFileType)
    }
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

fn read_from_file(path: &Path) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let path = Path::new(&args.path);

    if let Err(err) = Args::validate_file(path) {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    };

    let content = read_from_file(path)?;
    // Todo: Don't unwrap()
    let cards = Card::card_parser(content).unwrap();

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

        if let Event::Key(key) = event::read()? {
            match key.code {
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
                                UserAnswer::Correct => app_state.correct_answers += 1,
                                UserAnswer::Incorrect => app_state.incorrect_answers += 1,
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
                                    app_state.correct_answers += 1
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
                                    app_state.incorrect_answers += 1
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
