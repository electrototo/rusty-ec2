use ratatui::{
    Frame,
    prelude::{
        Layout,
        Constraint,
        Direction,
        Style,
        Stylize
    },
    style::Color,
    widgets::{
        Block,
        Paragraph,
        Borders,
        List,
        ListItem
    },
    text::{Text, Line}
};

use aws_sdk_ec2::types::Reservation;

use crate::app::{App, LocalReservation};

pub fn ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3)
        ])
        .split(frame.area());

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Available EC2 instances",
        Style::default().fg(Color::Green)
    ))
    .block(title_block);

    frame.render_widget(title, chunks[0]);

    let items: Vec<ListItem> = app
        .reservations
        .iter()
        .map(|reservation| {
            ListItem::from(reservation)
        })
        .collect();

    let list_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let list = List::new(items)
        .block(list_block);

    frame.render_widget(list, chunks[1]);
}

impl From<&LocalReservation> for ListItem<'_> {
    fn from(value: &LocalReservation) -> Self {
        let instance_id = value.reservation.instances().first().unwrap().instance_id().unwrap().to_string();
        let line = match value.selected {
            true => Line::styled(format!("{}", instance_id), Style::default().bg(Color::White)),
            false => Line::styled(format!("{}", instance_id), Style::default())
        };

        ListItem::new(line)
    }
}