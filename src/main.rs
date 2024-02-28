mod telegram;

use std::io::{self, Write};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent},
    terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType},
    execute, queue,
};

use crate::telegram::Client;

#[derive(Debug)]
enum AppError {
    Unknown,

    LoginFailed,
}

fn read_ch() -> char {
    loop {
        if let Ok(Event::Key(KeyEvent {
            code: KeyCode::Char(c),
            ..
        })) = event::read()
        {
            return c;
        }
    }
}

fn app_main() -> Result<(), AppError> {
    let mut stdout = io::stdout();
    
    execute!(stdout, Clear(ClearType::All));

    let client = Client::new();

    let user = if client.logged_in().map_err(|_| AppError::Unknown)? {
        client.get_user().map_err(|_| AppError::Unknown)
    } else {
        let number = read_ch();

        queue!(stdout, cursor::MoveTo(0, 0));
        print!("{}", number);
        stdout.flush();

        return Err(AppError::Unknown);
    }?;

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
            None => "".to_string()
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

    Ok(())
}

fn main() {
    enable_raw_mode();

    let res = app_main();

    disable_raw_mode();

    match res {
        Ok(_) => println!("\nExited successfully."),
        Err(e) => println!("\nExited with error: {e:?}")
    };
}
