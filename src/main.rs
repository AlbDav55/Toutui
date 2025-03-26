mod login_app;
mod app;
mod config;
mod api;
mod ui;
mod player;
mod logic;
mod db;
mod utils;

use login_app::AppLogin;
use app::App;
use crate::db::database_struct::Database;
use color_eyre::Result;
use std::time::Duration;
use crossterm::event::{self, KeyCode};
use std::io::stdout;
use crate::utils::pop_up_message::*;
use crate::utils::logs::*;
use log::info;
use crate::db::crud::*;
use ratatui::{
    style::{Color, Style},
    widgets::Block
};
use crate::player::integrated::player_info::*;
use crate::ui::player_tui::*;
use std::env;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {

    // this function allow to write all the logs in a file 
    setup_logs().expect("Failed to execute logger");

    // set dotenv to ~/.config.toutui/.env for linux
    // Library/Application Support/toutui/.env for macos
    // (dotenv will be use in `encrypt_token.rs`)
    let home_dir = dirs::home_dir().expect("Unable to find the user's home directory");
    // if env::var("XDG_CONFIG_HOME") is not empty env_path will take designed path
    // else, env_path will be set to default path
    let env_path = env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)  // Converts the String to PathBuf
        .unwrap_or_else(|_| { 
            if cfg!(target_os = "macos") {
            // If XDG_CONFIG_HOME is not defined on macOS, use the default directory
            home_dir.join("Library").join("Preferences")
        } else {
            // Otherwise, use ~/.config for other systems (like Linux)
            home_dir.join(".config")
        }
        });
    // Construct the dotenv 
    dotenv::from_filename(&env_path.clone()).ok();

    // Init database
    let mut _database = Database::new().await?;
    let mut _database_ready = false;

    // Wait for the database to be ready, waiting for the user to enter their credentials
    loop {
        _database = Database::new().await?;
        if _database.default_usr.is_empty() {
            let app_login = AppLogin::new().await?;
            let terminal = ratatui::init();
            let _app_result = app_login.run(terminal);
            // Process login result here
            // Wait for 1 second before checking again
            // If database is reinit to quickly before `auth_process.rs` is finished
            // it can be buggy and mark as failed. Maybe add more time to be sure (like 6 sec).
            // But normally, even it's failed, data are written in db. It will work at the second
            // attempt...
            tokio::time::sleep(Duration::from_secs(1)).await;
        } else {
            // If the database is ready, exit the loop
            print!("\x1B[2J\x1B[1;1H"); // clear all stdout (avoid to sill have the previous print when the app is launched)
            _database_ready = true;
            info!("Database ready");
            break;
        }
    }

    // Once the database is ready, initialize the app
    if _database_ready {

        // init current username
        let mut username: String = String::new();
        if let Some(var_username) = _database.default_usr.get(0) {
            username = var_username.clone();
        }
        // init is_vlc_launched_first_time 
        let _ = update_is_vlc_launched_first_time("1", username.as_str());
        let value = get_is_vlc_launched_first_time(username.as_str());
        info!("[main][is_vlc_launched_first_time] {}", value);

        let mut app = App::new().await?;
        let mut terminal = ratatui::init();

        // Running the app in a loop
        loop {

            let is_playing = get_is_vlc_running(app.username.as_str());
            let player_info = player_info(app.username.as_str());

            terminal.draw(|frame| {
                let bg_color = app.config.colors.background_color.clone();
                let bg_color_player = app.config.colors.player_background_color.clone();
                // global background
                let background = Block::default()
                    .style(Style::default()
                        .bg(Color::Rgb(bg_color[0], bg_color[1], bg_color[2])));

                frame.render_widget(background, frame.area());

                if is_playing == "1" {
                    let area = frame.area();
                    // render for the player (automatically refreshed) 
                    render_player(area, frame.buffer_mut(), player_info, bg_color_player, app.username.as_str()); 
                }

                // render widget for general app : 
                // Will be manually refresh by pressing `R`
                // If `app` variable is reinitialized below (`app = App::new().await?`), it will be taken into account and data will be refreshed
                // Otherwise, the current `app` variable will still be used.
                frame.render_widget(&mut app, frame.area());
            })?;


            // Checking if any key is pressed (waiting for events with a 200ms delay here)
            if crossterm::event::poll(Duration::from_millis(200))? {
                if let event::Event::Key(key) = crossterm::event::read()? {
                    app.handle_key(key);
                    match key.code {
                        // If the 'R' key is pressed, refresh the app
                        KeyCode::Char('R') => {
                            // pop up message
                            let mut stdout = stdout();
                            let _ = clear_message(&mut stdout, 3); // clear a message, if any, before print the message bellow
                            let _ = pop_message(&mut stdout, 3, "Refreshing app...");
                            // Reinitialize app to refresh
                            app = App::new().await?; 
                            // clear message above
                            let _ = clear_message(&mut stdout, 3);
                        }
                        _ => {}
                    }
                }
            }

            // Short pause between event checks
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }

    // Restore the terminal state before exiting the application
    ratatui::restore();
    Ok(())
}
