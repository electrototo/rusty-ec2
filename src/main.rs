use app::App;

use aws_config::meta::region::RegionProviderChain;
use aws_config::profile::ProfileFileCredentialsProvider;
use aws_config::{BehaviorVersion, SdkConfig};
use aws_sdk_ec2::{Client, Error};

use ratatui::Terminal;
use ratatui::crossterm::{execute, event};
use ratatui::crossterm::event::{Event, KeyEvent, KeyCode};
use ratatui::crossterm::terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::prelude::{CrosstermBackend, Backend};

use std::{io, env, process::Command};
use ui::ui;

use crate::app::LocalReservation;

mod app;
mod ui;


#[tokio::main]
async fn main() -> Result<(), Error>{
    // obtain from the args the profile to be used
    let args: Vec<String> = env::args().collect();

    if (args.len() != 2) {
        panic!("Usage: {} profile_name", args.get(0).unwrap());
    }

    let profile_name = args.get(1)
        .expect("Could not retrieve the profile name");

    let region_provider: RegionProviderChain = RegionProviderChain::default_provider()
        .or_else("us-east-1");

    let config: SdkConfig= aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .credentials_provider(ProfileFileCredentialsProvider::builder()
            .profile_name(profile_name)
            .build())
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
        selected_instance: 0,
        aws_profile_name: profile_name.to_string()
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

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<bool> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            // Remove the previously selected instance bg color
            app.reservations.get_mut(app.selected_instance).unwrap().selected = false;

            match key.code {
                KeyCode::Char('k') | KeyCode::Up => {
                    app.selected_instance = app.selected_instance.saturating_sub(1);
                },
                KeyCode::Char('j') | KeyCode::Down => {
                    if app.selected_instance + 1 < app.reservations.len() {
                        app.selected_instance = app.selected_instance.saturating_add(1)
                    }
                },
                KeyCode::Enter => {
                    let ec2_instance = app
                        .reservations
                        .get(app.selected_instance)
                        .unwrap()
                        .reservation
                        .instances()
                        .first()
                        .unwrap()
                        .instance_id()
                        .unwrap();

                    Command::new("gnome-terminal")
                        .arg("--")
                        .arg("aws")
                        .arg("ssm")
                        .arg("start-session")
                        .arg("--target")
                        .arg(ec2_instance)
                        .arg("--profile")
                        .arg(app.aws_profile_name.clone())
                        // .arg(format!("aws ssm start-session --target {} --profile {}", ec2_instance, app.aws_profile_name))
                        .spawn();
                },
                KeyCode::Char('q') => app.exit = true,
                _ => {}
            }

            // Color the selected row, that includes information related to the EC2 instance
            app.reservations.get_mut(app.selected_instance).unwrap().selected = true;
        }

        if app.exit {
            return Ok(true);
        }
    }
}