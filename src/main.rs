use app::App;
use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_ec2::{Client, Error};

use ratatui::Terminal;
use ratatui::crossterm::{execute, event};
use ratatui::crossterm::event::{Event, KeyEvent, KeyCode};
use ratatui::crossterm::terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::prelude::{CrosstermBackend, Backend};
use std::io;
use ui::ui;

use crate::app::LocalReservation;

mod app;
mod ui;


#[tokio::main]
async fn main() -> Result<(), Error>{
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;

    let client = Client::new(&config);

    let response = client.describe_instances()
        .send()
        .await;

    let reservations = response.unwrap().reservations.unwrap();

    // Initialize the application state after it was loaded from AWS
    let mut app = App {
        exit: false,
        reservations: reservations.iter().enumerate().map(|(index, reservation)| -> LocalReservation {
            LocalReservation {
                reservation: reservation.clone(),
                selected: index == 0
            }
        }).collect(),
        selected_instance: 0
    };

    // Run the application
    enable_raw_mode().unwrap();
    let mut stderr = io::stderr();

    execute!(stderr, EnterAlternateScreen);

    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend).unwrap();

    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode().unwrap();
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    ).unwrap();

    terminal.show_cursor().unwrap();

    // response.unwrap()
    //     .reservations()
    //     .iter()
    //     .for_each(|reservation| -> () {
    //         let instances = reservation.instances();

    //         instances
    //             .iter()
    //             .for_each(|instance| -> () {
    //                 let tags = instance.tags();

    //                 println!("Instance ID: {}", instance.instance_id().unwrap());
    //                 println!("Tags:");
    //                 tags.iter().for_each(|tag| -> () {
    //                     println!("\t{}: {}", tag.key().unwrap(), tag.value().unwrap());
    //                 });
    //             });
    //             println!("");
    //     });

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match key.code {
                KeyCode::Char('w') => {
                    app.reservations.get_mut(app.selected_instance).unwrap().selected = false;
                    app.selected_instance = app.selected_instance.saturating_sub(1);
                },
                KeyCode::Char('s') => {
                    app.reservations.get_mut(app.selected_instance).unwrap().selected = false;
                    if app.selected_instance + 1 < app.reservations.len() {
                        app.selected_instance = app.selected_instance.saturating_add(1)
                    }
                },
                KeyCode::Char('q') => app.exit = true,
                _ => {}
            }

            app.reservations.get_mut(app.selected_instance).unwrap().selected = true;
        }

        if app.exit {
            return Ok(true);
        }
    }
}