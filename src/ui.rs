use ratatui::{
    Frame,
    prelude::{
        Layout,
        Constraint,
        Direction,
        Style,
    },
    style::{Color, Modifier, Styled},
    widgets::{
        Block,
        Paragraph,
        Borders,
        List,
        ListItem,
        block::Padding
    },
    text::{Text, Line}
};


use crate::app::{App, LocalReservation};

const SELECTED_STYLE: Style = Style::new().add_modifier(Modifier::BOLD);

pub fn ui(frame: &mut Frame, app: &App) {
    // Title, main window, and footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3)
        ])
        .split(frame.area());

    // From the main window, side nav, and detail section
    let main_area = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(90)
            ])
            .split(chunks[1]);

    let title_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default());

    let title = Paragraph::new(Text::styled(
        "Available EC2 instances",
        Style::default().fg(Color::Green)
    ))
    .block(title_block);

    frame.render_widget(title, chunks[0]);

    // For every item in the list, get the styling for every each one of the list.
    // here we will set the styling, for the selected row
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

    // Render the sidenavar with the instances IDs 
    frame.render_widget(list, main_area[0]);


    // ============
    // Render the details of the instances, all the information taht can be obtained from the reservation
    // item on the main screen
    let selected_reservation = app.reservations.get(app.selected_instance)
        .expect("Invalid selected instance.");

    let selected_instance = selected_reservation.reservation.instances().first().unwrap();
    let instance_type = selected_instance.instance_type().unwrap();

    let mut tags: Vec<Line> = selected_instance.tags()
        .iter()
        .map(|tag| -> Line {
            Line::styled(format!("{}: {}", (*tag).key().unwrap(), (*tag).value().unwrap()), Style::default())
        })
        .collect();

    let instance_details_block = Block::default()
        .borders(Borders::ALL)
        .style(Style::default())
        .padding(Padding::new(1, 0, 0, 0));

    let mut tags_display: Vec<Line> = Vec::new();
    // tags_display.append(
    //     &mut Line::styled("Tags", Style::default().add_modifier(Modifier::BOLD))
    // );

    tags_display.append(&mut tags);

    let instance_details = List::new([
        Text::from(vec![
            Line::styled("Instance Size", Style::default().add_modifier(Modifier::BOLD)),
            Line::styled(instance_type.as_str(), Style::default()),
        ]),
        Text::from(vec![
            Line::styled("Instance State", Style::default().add_modifier(Modifier::BOLD)),
            Line::styled(selected_instance.state().unwrap().name().unwrap().as_str(), Style::default()),
        ]),
        Text::styled("Tags", Style::default().add_modifier(Modifier::BOLD)),
        Text::from(tags_display)
    ]).block(instance_details_block);

    // let instance_details = Paragraph::new(Text::from(vec![
    //     Line::styled("Tags", Style::default().add_modifier(Modifier::BOLD)),
    //     Line::styled(tags.join("\n"), Style::default()),
    // ]));

    frame.render_widget(instance_details, main_area[1]);
}

impl From<&LocalReservation> for ListItem<'_> {
    fn from(value: &LocalReservation) -> Self {
        let instance_id: String = value.reservation.instances().first().unwrap().instance_id().unwrap().to_string();

        let line = match value.selected {
            true => Line::styled(format!("> {}", instance_id), SELECTED_STYLE),
            false => Line::styled(format!("  {}", instance_id), Style::default())
        };

        ListItem::new(line)
    }
}