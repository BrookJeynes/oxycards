use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Layout},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::{models::card::Card, AppState};

pub fn ui<B: Backend>(f: &mut Frame<B>, app_state: &mut AppState) {
    let size = f.size();

    let create_block = |title: &str| {
        Block::default()
            .borders(Borders::ALL)
            .title(title.to_string())
    };

    let chunks = Layout::default()
        .margin(1)
        .constraints([
            Constraint::Percentage(90),
            Constraint::Percentage(5),
            Constraint::Percentage(5),
        ])
        .split(size);

    let inner_card = Layout::default()
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(chunks[0]);

    match app_state.cards.selected() {
        Card::FlashCard(card) => {
            let question = Paragraph::new(card.question.to_string())
                .block(create_block("Question"))
                .alignment(Alignment::Center);

            let answer = Paragraph::new(if card.flipped {
                card.answer.to_string()
            } else {
                String::new()
            })
            .block(create_block("Answer"))
            .alignment(Alignment::Center);

            f.render_widget(question, inner_card[0]);
            f.render_widget(answer, inner_card[1]);
        }
        Card::MultipleChoice(card) => {
            let question = Paragraph::new(card.question.to_string())
                .block(create_block("Question"))
                .alignment(Alignment::Center);

            let choices: Vec<ListItem> = card
                .choices
                .items
                .iter()
                .map(|choice| ListItem::new(choice.as_ref()))
                .collect();

            let choices_list = List::new(choices)
                .block(create_block("Choices"))
                .highlight_symbol("> ");

            f.render_widget(question, inner_card[0]);
            f.render_stateful_widget(choices_list, inner_card[1], &mut card.choices.state);
        }
        Card::MultipleAnswer(card) => {
            let question = Paragraph::new(card.question.to_string())
                .block(create_block("Question"))
                .alignment(Alignment::Center);

            let choices: Vec<ListItem> = card
                .choices
                .items
                .iter()
                .map(|choice| {
                    ListItem::new(format!(
                        "[{}] {}",
                        if choice.selected { "x" } else { " " },
                        choice.content.to_string()
                    ))
                })
                .collect();

            let choices_list = List::new(choices)
                .block(create_block("Choices"))
                .highlight_symbol("> ");

            f.render_widget(question, inner_card[0]);
            f.render_stateful_widget(choices_list, inner_card[1], &mut card.choices.state);
        }
        Card::FillInTheBlanks(card) => {
            let question = Paragraph::new(card.question.to_string())
                .block(create_block("Question"))
                .alignment(Alignment::Center);

            f.render_widget(question, inner_card[0]);
        }
        Card::Order(card) => {
            let question = Paragraph::new(card.question.to_string())
                .block(create_block("Question"))
                .alignment(Alignment::Center);

            f.render_widget(question, inner_card[0]);
        }
    };

    f.render_widget(create_block("Controls"), chunks[2]);
}
