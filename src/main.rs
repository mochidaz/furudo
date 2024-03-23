use std::sync::{Arc, mpsc, Mutex, RwLock};
use std::{thread, time};
use std::time::Duration;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent};

use rand::Rng;
use crossterm::terminal::{Clear, ClearType::All};

#[derive(PartialEq)]
enum Status {
    Running,
    Stopped,
}

struct FloatingText {
    text: String,
    x: u16,
    y: u16,
    speed: u16,
    last_update: time::Instant,
}

impl FloatingText {
    fn new(text: &str, x: u16, y: u16, speed: u16) -> Self {
        Self {
            text: text.to_string(),
            x,
            y,
            speed,
            last_update: time::Instant::now(),
        }
    }

    fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    fn update(&mut self) -> bool {
        let elapsed = self.last_update.elapsed();
        if elapsed.as_millis() >= self.speed as u128 {
            self.last_update = time::Instant::now();
            true
        } else {
            false
        }
    }

    fn clear(&self) {
        print!("\x1B[{};{}H\x1B[K", self.y, self.x);
    }
}

fn clear_screen() {
    println!("{}", Clear(All));
}

fn print_ascii(ascii: &str, x: u16, y: u16) {
    let mut y = y;
    for line in ascii.lines() {
        print!("\x1B[{};{}H{}", y, x, line);
        y += 1;
    }
}

fn send_texts(texts: &Arc<Mutex<Vec<Arc<Mutex<FloatingText>>>>>, messages: &[&str], size: (u16, u16), amount: u16) {
    let mut texts = texts.lock().unwrap();
    for _ in 0..amount {
        let text = messages[generate_random_range(0, messages.len() as u16) as usize];
        let mut generate_y = generate_random_range(0, size.1);

        while texts.iter().any(|text| text.lock().unwrap().y == generate_y) {
            generate_y = generate_random_range(0, size.1);
        }

        texts.push(Arc::new(
            Mutex::new(
                FloatingText::new(
                    text,
                    size.0,
                    generate_y,
                    generate_random_range(20, 70
                )
            )
        )));
    }
}

fn generate_random_range(min: u16, max: u16) -> u16 {
    rand::thread_rng().gen_range(min..max)
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
                print!(
                    "\x1B[{};{}H{}\x1B[K",
                    text.y, text.x, text.text
                );

                text.move_left();

                if text.x == 0 {
                    text.clear();
                }
            }
        }

        vec.retain(|text| text.lock().unwrap().x > 0);

        if *status.read().unwrap() == Status::Stopped {
            break;
        }
    }

    if sender.join().is_ok() && event_handler.join().is_ok() {
        clear_screen();
    }
}