use std::sync::{Arc, Mutex};

use crossterm::terminal::Clear;
use crossterm::terminal::ClearType::All;

use rand::Rng;

use crate::text::FloatingText;

pub fn clear_screen() {
    println!("{}", Clear(All));
}

pub fn print_ascii(ascii: &str, x: u16, y: u16) {
    let mut y = y;
    for line in ascii.lines() {
        print_exact(x, y, line);
        y += 1;
    }
}

pub fn print_exact(x: u16, y: u16, text: &str) {
    print!("\x1B[{};{}H{}", y, x, text);
}

pub fn send_texts(texts: &Arc<Mutex<Vec<FloatingText>>>, messages: &[&str], size: (u16, u16), amount: u16) {
    let mut texts = texts.lock().unwrap();
    for _ in 0..amount {
        let text = messages[generate_random_range(0, messages.len() as u16) as usize];
        let mut generate_y = generate_random_range(0, size.1);

        while texts.iter().any(|text| text.height() == generate_y) {
            generate_y = generate_random_range(0, size.1);
        }

        texts.push(
            FloatingText::new(
                text,
                size.0,
                generate_y,
                generate_random_range(20, 70
                )
            )
        );
    }
}

fn generate_random_range(min: u16, max: u16) -> u16 {
    rand::thread_rng().gen_range(min..max)
}