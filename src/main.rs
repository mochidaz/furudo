use std::sync::{Arc, Mutex};
use std::{thread, time};
use rand::Rng;

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
    println!("{}", termion::clear::All);
}

fn print_ascii(ascii: &str, x: u16, y: u16) {
    let mut y = y;
    for line in ascii.lines() {
        print!("\x1B[{};{}H{}", y, x, line);
        y += 1;
    }
}

fn send_texts(texts: &mut Vec<Arc<Mutex<FloatingText>>>, messages: &Vec<&str>, size: (u16, u16)) {
    for _ in 0..15 {
        let text = messages[generate_random_range(0, messages.len() as u16) as usize];
        let mut generate_y = generate_random_range(0, size.1);

        while texts.iter().any(|text: &Arc<Mutex<FloatingText>>| text.lock().unwrap().y == generate_y) {
            generate_y = generate_random_range(0, size.1);
        }

        texts.push(Arc::new(Mutex::new(FloatingText::new(text, size.0, generate_random_range(0, size.1), generate_random_range(20, 70)))));
    }
}

fn generate_random_range(min: u16, max: u16) -> u16 {
    rand::thread_rng().gen_range(min..max)
}

fn main() {
    clear_screen();

    let erika = "
    ⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣠⣾⣿⣷⣶⡀
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

    let mut texts = Vec::new();

    let size = termion::terminal_size().unwrap();

    send_texts(&mut texts, &messages, size);

    let mut handles = vec![];

    let handle = thread::spawn(move || {
        loop {
            print_ascii(erika, size.0 / 4, size.1 / 9);

            for text in &texts {
                let mut text = text.lock().unwrap();
                if text.update() {
                    print!(
                        "\x1B[{};{}H{}\x1B[K",
                        text.y, text.x, text.text
                    );

                    text.move_left();

                    if text.x == 0 {
                        text.clear();
                        text.x = size.0;
                        text.y = generate_random_range(0, size.1);
                        text.speed = generate_random_range(20, 70);
                    }
                }
            }
            thread::sleep(time::Duration::from_millis(10));
        }
    });

    handles.push(handle);

    for handle in handles {
        handle.join().unwrap();
    }
}
