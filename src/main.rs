mod text;
mod utils;

use std::sync::{Arc, Mutex, RwLock};
use std::{thread};
use std::time::Duration;

use crossterm::event;
use crossterm::event::{Event, KeyCode};

use rand::Rng;

use crate::utils::{clear_screen, print_ascii, send_texts};

#[derive(PartialEq)]
enum Status {
    Running,
    Stopped,
}

fn main() {
    clear_screen();

    let erika = "
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣾⣿⣷⣶⡀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⠀⢠⣿⣿⣿⣿⣿⣿⣷
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣴⣾⣥⡙⠛⢿⣿⠿⢿⣿⣿⠀⢶⣿⣶⣄
⠀⠀⠀⠀⠀⠀⠀⢠⣾⣿⣿⣿⣿⣿⣿⣿⣿⣶⣼⡀⠀⡀⠉⠀⠀⡈⣿⣿⣿⡆
⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣷⣦⣀⣀⣼⠛⠛⢿⣷⡆
⠀⠀⠀⠀⠀⠀⠀⠸⣿⣿⣿⣿⣿⣿⣿⣿⠿⡿⢿⣿⣿⣿⣿⣿⣿⣿⡂⠢⡑⠌⠙
⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣿⡿⠛⣓⡀⠀⠀⠐⢻⣯⠇⣿⣿⡏⢹⣿⣷⠐⢝⠦⠜
⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣿⣦⡈⠹⠿⠀⠀⠀⠈⠁⠀⣿⣿⣷⣸⣿⣿⡄⠀⣁⣠⣿⡀
⠀⠀⠀⠀⠀⠀⠀⣼⣿⣿⣿⣿⣷⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣷⡈⢦⣌⠻⣧
⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⣿⣦⡀⠀⠀⠀⠀⠀⠠⣿⣿⣿⣿⠟⠁⢻⣿⣄⡈⢿⣮⣃
⠀⠀⠀⠀⠀⠀⢰⣿⣿⣿⣿⣿⣿⢿⣿⣿⣶⡤⠀⠀⠀⣿⣿⣏⠀⠀⠀⠈⠿⣿⣿⣄⠙⣿⡇
⠀⠀⠀⠀⠀⠀⣾⣿⣿⣿⣿⣿⣿⡀⠈⠙⠀⠀⠀⠀⢰⣿⣿⡇⠀⠀⠀⠀⠀⠀⠈⠻⣷⣌⢻⡄
⠀⠀⠀⠀⠀⢸⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠠⠀⡀⢸⣿⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠸⣿⣦⡁⠀⠆
⠀⠀⠀⠀⠀⣾⣿⣿⣿⣿⡟⣿⣿⣧⠀⠘⠛⠂⠀⣀⣿⣿⣿⣯⠀⠀⠀⠀⠀⠀⠀⠀⠈⣿⣿⣿⡄
⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⡇⢹⣿⣿⣆⣠⣾⠏⠀⢸⣿⣿⣿⣿⠄⠀⠀⠀⠀⠀⠀⠀⢠⣏⣿⣿⣿⡄
⠀⠀⠀⠀⠀⣿⣿⣿⣿⣿⡇⠸⣿⣿⣿⣉⢼⣷⣶⢸⣿⣿⣿⣧⣀⠀⠀⠀⠀⠀⠀⢰⣿⣿⣽⣿⣿⡇⠀⡀
⠀⠀⠀⠀⢠⣿⣿⣿⣿⣿⡇⠀⢻⣿⢿⢳⣿⣿⣿⣾⣿⡟⣿⣿⣿⡧⠀⠀⠀⠀⠀⠰⣿⣿⢸⣿⣿⣿
⠀⠀⠀⠀⠸⣿⣿⣿⣿⣇⡀⠀⣈⡟⢸⠛⣿⣿⣿⣿⢿⡇⣿⣿⣿⡇⡀⠀⠀⠀⠀⠀⣿⣿⡇⣿⣿⠇
⠀⠀⠀⠀⠀⣿⣿⣿⣟⠃⠀⠀⠙⠼⠭⠦⠝⠛⠛⠓⠊⠉⠁⠀⢻⣿⠁⠀⠀⠀⠀⣤⣿⣿⡇⡿⠃
⠀⠀⠀⠀⠀⣿⣷⣄⡀⢚⣓⠠⠀⠀⠀⣠⠀⠀⠲⡦⡀⠉⠀⠉⠉⣯⠀⠀⡀⠀⠀⣿⣿⡿⠃⠁
⠀⠀⠀⠠⠀⠾⠿⠋⠉⣁⣀⣀⣴⣶⣿⣿⣷⡀⠀⠈⠐⠠⣀⣀⣀⡿⠁⠀⠀⠀⠀⣿⣿⣦⢀
⠀⠀⠀⠀⠀⠈⠛⢛⣯⣽⣷⣿⣿⣿⣿⡟⡍⡩⢦⡀⠀⠀⠈⠉⡛⠏⠀⠀⠀⠀⠀⢹⣿⣿⣿⠂
⠀⠀⠀⣀⠀⠀⠀⢘⣿⣿⣿⣿⣿⣿⣿⡟⠀⣴⡁⠙⢶⣤⣀⢀⠏⠀⠀⠀⠀⠀⠀⠈⣿⣿⣿⣇
⠀⠐⠀⡿⠳⣶⣴⣾⣿⣿⣿⣿⣿⣿⣿⣧⣠⠏⠨⡠⠀⠙⢿⠟⠀⠀⠀⠀⠀⠀⠀⠀⠹⣿⣿⣿
⢁⣀⣴⠃⠥⣈⣹⣿⣿⣿⣿⣿⡿⠋⠀⣿⣿⠀⢀⠟⠒⠄⠀⠄⠀⠀⠀⠀⠀⠀⠀⠀⠀⣿⣿⣿⣷⣄";

    let messages = vec![
        "Furudo Erika is so cute!",
        "I love Furudo Erika!",
        "Furudo Erika is the best!",
        "Furudo Erika is the greatest detective!",
        "I love the detective from Episode 5!",
        "<Very good> I like Furudo Erika!",
        "I want to be with Furudo Erika!",
        "Furudo Erika is my favorite!",
        "Furudo Erika is gorgeous!",
        "Furudo Erika is an amazing detective!",
        "I like Furudo Erika so much!",
        "Furudo Erika best girl!",
        "<Oh yeaaaaaaaaaaaah! Veeeeeerrryyyyy goooooodd!!!>",
        "<Good>",
    ];

    let size = crossterm::terminal::size().unwrap();

    let texts = Arc::new(Mutex::new(Vec::new()));

    let sender_queue = Arc::clone(&texts);

    let mut status = Arc::new(RwLock::new(Status::Running));

    let sender_status = Arc::clone(&status);

    let sender = thread::spawn(move || {
        let status = Arc::clone(&sender_status);

        loop {
            send_texts(&sender_queue, &messages, size, 1);
            thread::sleep(Duration::from_secs(1));

            if *status.read().unwrap() == Status::Stopped {
                break;
            }
        }
    });

    let receiver_vector = Arc::clone(&texts);

    let receiver_status = Arc::clone(&status);

    let event_handler = thread::spawn(move || {
        loop {
            if let Event::Key(event) = event::read().unwrap() {
                match event.code {
                    KeyCode::Enter => {
                        *status.write().unwrap() = Status::Stopped;
                        break;
                    }
                    _ => {}
                }
            }
        }
    });

    loop {
        let status = Arc::clone(&receiver_status);

        print_ascii(erika, size.0 / 4, size.1 / 9);

        let mut vec = receiver_vector.lock().unwrap();

        for text in vec.iter_mut() {
            let mut text = text.lock().unwrap();
            if text.update() {
                text.print();

                text.move_left();

                if text.has_ended() {
                    text.clear();
                }
            }
        }

        vec.retain(|text| text.lock().unwrap().width() > 0);

        if *status.read().unwrap() == Status::Stopped {
            break;
        }
    }

    if sender.join().is_ok() && event_handler.join().is_ok() {
        clear_screen();
    }
}