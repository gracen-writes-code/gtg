mod telegram;

use std::io;

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use crate::telegram::Client;

fn main() {
    let client = Client::new();

    let user = if client.logged_in().unwrap() {
        client.get_user().unwrap()
    } else {
        todo!() // log the client in
    };

    // let stdin = io::stdin();

    // let user = if !client.is_authorized().await.unwrap() {
    //     let mut number = String::new();
    //     println!("Input phone number: ");
    //     stdin.read_line(&mut number).unwrap();

    //     let token = client.request_login_code(&number).await.unwrap();

    //     let mut code = String::new();
    //     println!("Input verification code: ");
    //     stdin.read_line(&mut code).unwrap();

    //     client.sign_in(&token, &code).await.unwrap()
    // } else {
    //     client.get_me().await.unwrap()
    // };

    client.save_session();

    println!("Logged in as [ {} ].", user.full_name());

    let dialogs = client.get_dialogs().unwrap();

    for dialog in dialogs {
        println!("{}: ({})", dialog.name(), match dialog.last_message_text() {
            Some(m) => m,
            None => ""
        });
    }

    // let mut iter_dialogs = client.iter_dialogs();
    // let mut dialogs: Vec<&Dialog> = vec![];

    // println!("Printing available dialogs:");
    // while let Some(dialog) = &iter_dialogs.next().await.unwrap() {
    //     match &dialog.chat {
    //         Chat::User(user) => println!(" - DM with [ {} ]: {}", user.full_name(), dialog.chat.id()),
    //         Chat::Group(_) => todo!(),
    //         Chat::Channel(_) => todo!(),
    //     }
    // }
}
