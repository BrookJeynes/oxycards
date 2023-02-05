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
    models::{
        fill_in_the_blanks::FillInTheBlanks,
        flashcard::FlashCard,
        multiple_answer:: MultipleAnswer,
        multiple_choice::MultipleChoice,
        order::Order,
    },
    AppState
};

pub fn ui<B: Backend>(f: &mut Frame<B>, app_state: &mut AppState) {
    let size = f.size();

    let create_block = |title: &str| {
        Block::default()
            .borders(Borders::ALL)
            .title(title.to_string())
    };

    let chunks = Layout::default()
        .horizontal_margin(2)
        .constraints([
            Constraint::Percentage(90),
            Constraint::Percentage(5),
            Constraint::Percentage(5),
        ])
        .split(size);

    let inner_card = Layout::default()
        .margin(size.area() / 800)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(chunks[0]);

    let content_layout = Layout::default()
        .horizontal_margin(size.area() / 800)
        .constraints([Constraint::Percentage(90), Constraint::Percentage(10)])
        .split(inner_card[1]);

    let incorrect = Paragraph::new(Span::styled(
        app_state.incorrect_answers.to_string(),
        Style::default().fg(Color::Red),
    ))
    .alignment(Alignment::Left);

    let correct = Paragraph::new(Span::styled(
        app_state.correct_answers.to_string(),
        Style::default().fg(Color::Green),
    ))
    .alignment(Alignment::Right);

    let cards = Paragraph::new(format!(
        "{}/{}",
        app_state.cards.current_card + 1,
        app_state.cards.cards.len()
    ))
    .alignment(Alignment::Center);

    match app_state.cards.selected() {
        Card::FlashCard(card) => {
            let question = Paragraph::new(card.question.to_string())
                .block(create_block("Question"))
                .wrap(Wrap { trim: false })
                .alignment(Alignment::Center);

            let answer = Paragraph::new(if card.flipped {
                card.answer.to_string()
            } else {
                String::new()
            })
            .block(create_block("Answer"))
            .wrap(Wrap { trim: false })
            .alignment(Alignment::Center);

            let controls = Paragraph::new(FlashCard::instructions()).alignment(Alignment::Left);

            f.render_widget(question, inner_card[0]);
            f.render_widget(answer, inner_card[1]);
            f.render_widget(controls, chunks[2]);
        }
        Card::MultipleChoice(card) => {
            let question = Paragraph::new(card.question.to_string())
                .block(create_block("Question"))
                .wrap(Wrap { trim: false })
                .alignment(Alignment::Center);

            let choices: Vec<ListItem> = card
                .choices
                .items
                .iter()
                .map(|choice| {
                    // Todo: Please for the love of god make this better
                    ListItem::new({
                        if choice.selected {
                            Span::styled(
                                choice.content.to_string(),
                                if let Some(value) = card.correct_answer {
                                    if value {
                                        Style::default().fg(Color::Green)
                                    } else {
                                        Style::default().fg(Color::Red)
                                    }
                                } else {
                                    Style::default().fg(Color::Blue)
                                },
                            )
                        } else {
                            Span::styled(
                                choice.content.to_string(),
                                if let Some(value) = card.correct_answer {
                                    if card.answers[0] == choice.content && !value {
                                        Style::default().fg(Color::Green)
                                    } else {
                                        Style::default()
                                    }
                                } else {
                                    Style::default()
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

            f.render_widget(question, inner_card[0]);
            f.render_stateful_widget(choices_list, inner_card[1], &mut card.choices.state);
            f.render_widget(controls, chunks[2]);
        }
        Card::MultipleAnswer(card) => {
            let question = Paragraph::new(card.question.to_string())
                .block(create_block("Question"))
                .wrap(Wrap { trim: false })
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

            let controls =
                Paragraph::new(MultipleAnswer::instructions()).alignment(Alignment::Left);

            f.render_widget(question, inner_card[0]);
            f.render_stateful_widget(choices_list, inner_card[1], &mut card.choices.state);
            f.render_widget(controls, chunks[2]);
        }
        Card::FillInTheBlanks(card) => {
            let question = Paragraph::new(card.question.to_string())
                .block(create_block("Question"))
                .wrap(Wrap { trim: false })
                .alignment(Alignment::Center);

            let content = Paragraph::new(card.content.to_string())
                .block(create_block("Content"))
                .wrap(Wrap { trim: false })
                .alignment(Alignment::Center);

            let controls = Paragraph::new(FillInTheBlanks::instructions()).alignment(Alignment::Left);

            f.render_widget(controls, chunks[2]);
            f.render_widget(question, inner_card[0]);
            f.render_widget(content, inner_card[1]);
        }
        Card::Order(card) => {
            let question = Paragraph::new(card.question.to_string())
                .block(create_block("Question"))
                .wrap(Wrap { trim: false })
                .alignment(Alignment::Center);

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
                                Span::styled(
                                    format!("{}", choice.content.to_string()),
                                    Style::default().fg(Color::Blue),
                                ),
                            ])
                        } else {
                            Spans::from(vec![Span::raw(format!(
                                "{}. {}",
                                i + 1,
                                choice.content.to_string()
                            ))])
                        }
                    })
                })
                .collect();

            let choices_list = List::new(choices)
                .block(create_block("Choices"))
                .highlight_symbol("> ");

            let controls = Paragraph::new(Order::instructions()).alignment(Alignment::Left);
            f.render_widget(question, inner_card[0]);
            f.render_stateful_widget(choices_list, inner_card[1], &mut card.shuffled.state);
            f.render_widget(controls, chunks[2]);
        }
    };

    f.render_widget(incorrect, content_layout[1]);
    f.render_widget(cards, content_layout[1]);
    f.render_widget(correct, content_layout[1]);
}
