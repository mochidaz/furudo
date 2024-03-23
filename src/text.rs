use std::time;

pub struct FloatingText {
    text: String,
    x: u16,
    y: u16,
    speed: u16,
    last_update: time::Instant,
}

impl FloatingText {
    pub fn new(text: &str, x: u16, y: u16, speed: u16) -> Self {
        Self {
            text: text.to_string(),
            x,
            y,
            speed,
            last_update: time::Instant::now(),
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn update(&mut self) -> bool {
        let elapsed = self.last_update.elapsed();
        if elapsed.as_millis() >= self.speed as u128 {
            self.last_update = time::Instant::now();
            true
        } else {
            false
        }
    }

    pub fn print(&self) {
        print!(
            "\x1B[{};{}H{}\x1B[K",
            self.y, self.x, self.text
        );
    }

    pub fn has_ended(&self) -> bool {
        self.x == 0
    }

    pub fn height(&self) -> u16 {
        self.y
    }

    pub fn width(&self) -> u16 {
        self.x
    }

    pub fn clear(&self) {
        print!("\x1B[{};{}H\x1B[K", self.y, self.x);
    }
}