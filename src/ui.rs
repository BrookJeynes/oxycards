use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::{
    models::card::Card,
    models::card_types::{
        fill_in_the_blanks::FillInTheBlanks,
        flashcard::FlashCard,
        multiple_answer:: MultipleAnswer,
        multiple_choice::MultipleChoice,
        order::Order,
    },
    AppState
};

pub fn ui<B: Backend>(f: &mut Frame<B>, app_state: &mut AppState) {
    let mut card_question = String::new();

    let size = f.size();

    // A helper closure to create blocks
    let create_block = |title: &str| {
        Block::default()
            .borders(Borders::ALL)
            .title(title.to_string())
    };

    // A helper closure to create styled spans
    let create_styled_span = |content: &str, colour: Color| -> Span {
        Span::styled(content.to_string(), Style::default().fg(colour))
    };

    // The main canvas
    let chunks = Layout::default()
        .horizontal_margin(2)
        // Card (90%), spacer (5%), controls (5%)
        .constraints([
            Constraint::Percentage(90),
            Constraint::Percentage(5),
            Constraint::Percentage(5),
        ])
        .split(size);

    // The area held for the card
    let card_layout = Layout::default()
        .margin(size.area() / 800)
        // Title (30%) and content (70%)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(chunks[0]);

    // The area held within each card
    let inner_card_layout = Layout::default()
        .horizontal_margin(size.area() / 800)
        // Content (90%) and card footer (10%)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
        .split(card_layout[1]);

    // Create card footer content
    let incorrect = Paragraph::new(create_styled_span(
        app_state.incorrect_answers.to_string().as_ref(),
        Color::Red,
    ))
    .alignment(Alignment::Left);

    let cards = Paragraph::new(format!(
        "{}/{}",
        app_state
            .cards
            .selected()
            .expect("This should never be None when this is called.")
            + 1,
        app_state.cards.items.len()
    ))
    .alignment(Alignment::Center);

    let correct = Paragraph::new(Span::styled(
        app_state.correct_answers.to_string(),
        Style::default().fg(Color::Green),
    ))
    .alignment(Alignment::Right);

    if let Some(val) = app_state.cards.selected_value() {
        match val {
            Card::FlashCard(card) => {
                card_question = card.question.clone();

                let answer = Paragraph::new(if card.flipped {
                    card.answer.to_string()
                } else {
                    String::new()
                })
                .block(create_block("Answer"))
                .wrap(Wrap { trim: false })
                .alignment(Alignment::Center);

                let controls = Paragraph::new(FlashCard::instructions()).alignment(Alignment::Left);

                f.render_widget(answer, card_layout[1]);
                f.render_widget(controls, chunks[2]);
            }
            Card::MultipleChoice(card) => {
                card_question = card.question.clone();
                let choices: Vec<ListItem> = card
                    .choices
                    .items
                    .iter()
                    .map(|choice| {
                        ListItem::new({
                            if choice.selected {
                                create_styled_span(
                                    choice.content.as_ref(),
                                    if let Some(value) = card.correct_answer {
                                        if value {
                                            Color::Green
                                        } else {
                                            Color::Red
                                        }
                                    } else {
                                        Color::Blue
                                    },
                                )
                            } else {
                                create_styled_span(
                                    choice.content.as_ref(),
                                    if let Some(value) = card.correct_answer {
                                        if card.answers[0] == choice.content && !value {
                                            Color::Green
                                        } else {
                                            Color::White
                                        }
                                    } else {
                                        Color::White
                                    },
                                )
                            }
                        })
                    })
                    .collect();

                let choices_list = List::new(choices)
                    .block(create_block("Choices"))
                    .highlight_symbol("> ");

                let controls = Paragraph::new(MultipleChoice::instructions()).alignment(Alignment::Left);

                f.render_stateful_widget(choices_list, card_layout[1], &mut card.choices.state);
                f.render_widget(controls, chunks[2]);
            }
            Card::MultipleAnswer(card) => {
                card_question = card.question.clone();

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

                let controls = Paragraph::new(Order::instructions()).alignment(Alignment::Left);

                f.render_stateful_widget(choices_list, card_layout[1], &mut card.choices.state);
                f.render_widget(controls, chunks[2]);
            }
            Card::FillInTheBlanks(card) => {
                card_question = card.question.clone();

                let content = Paragraph::new(card.content.to_string())
                    .block(create_block("Content"))
                    .wrap(Wrap { trim: false })
                    .alignment(Alignment::Center);

                f.render_widget(content, card_layout[1]);
            }
            Card::Order(card) => {
                card_question = card.question.clone();

                let choices: Vec<ListItem> = card
                    .shuffled
                    .items
                    .iter()
                    .enumerate()
                    .map(|(i, choice)| {
                        ListItem::new({
                            if choice.selected {
                                Spans::from(vec![
                                    Span::raw(format!("{}. ", i + 1)),
                                    create_styled_span(
                                        format!("{}", choice.content).as_ref(),
                                        Color::Blue,
                                    ),
                                ])
                            } else {
                                Spans::from(vec![create_styled_span(
                                    format!("{}. {}", i + 1, choice.content).as_ref(),
                                    if let Some(value) = card.correct_answer {
                                        if value {
                                            Color::Green
                                        } else {
                                            Color::Red
                                        }
                                    } else {
                                        Color::White
                                    },
                                )])
                            }
                        })
                    })
                    .collect();

                let choices_list = List::new(choices)
                    .block(create_block("Choices"))
                    .highlight_symbol("> ");

                let controls = Paragraph::new(Order::instructions()).alignment(Alignment::Left);

                f.render_stateful_widget(choices_list, card_layout[1], &mut card.shuffled.state);
                f.render_widget(controls, chunks[2]);
            }
        }
    };

    // Render card title
    f.render_widget(
        Paragraph::new(card_question)
            .block(create_block("Question"))
            .wrap(Wrap { trim: false })
            .alignment(Alignment::Center),
        card_layout[0],
    );

    // Render card footer
    f.render_widget(incorrect, inner_card_layout[1]);
    f.render_widget(cards, inner_card_layout[1]);
    f.render_widget(correct, inner_card_layout[1]);
}
