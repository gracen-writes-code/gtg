use std::{env, io};

use grammers_client::{types::{Chat, Dialog}, Client, Config};
use grammers_session::Session;
use serde_json::Value;

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};

const SECRETS: &'static str = include_str!("secrets.json");

#[tokio::main]
async fn main() {
    let secrets: Value = serde_json::from_str(SECRETS).unwrap();
    let session_file = env::var("HOME").unwrap() + "/gtg.session";

    let client = Client::connect(Config {
        session: Session::load_file_or_create(session_file.clone()).unwrap(),
        api_id: secrets["api_id"].as_i64().unwrap() as i32,
        api_hash: secrets["api_hash"].as_str().unwrap().into(),
        params: Default::default(),
    })
    .await
    .unwrap();

    let stdin = io::stdin();

    let user = if !client.is_authorized().await.unwrap() {
        let mut number = String::new();
        println!("Input phone number: ");
        stdin.read_line(&mut number).unwrap();

        let token = client.request_login_code(&number).await.unwrap();

        let mut code = String::new();
        println!("Input verification code: ");
        stdin.read_line(&mut code).unwrap();

        client.sign_in(&token, &code).await.unwrap()
    } else {
        client.get_me().await.unwrap()
    };

    client.session().save_to_file(session_file).unwrap();

    println!("Logged in as [ {} ].", user.full_name());

    let mut iter_dialogs = client.iter_dialogs();
    let mut dialogs: Vec<&Dialog> = vec![];

    println!("Printing available dialogs:");
    while let Some(dialog) = &iter_dialogs.next().await.unwrap() {
        match &dialog.chat {
            Chat::User(user) => println!(" - DM with [ {} ]: {}", user.full_name(), dialog.id()),
            Chat::Group(_) => todo!(),
            Chat::Channel(_) => todo!(),
        }
    }
}
