use ratatui::crossterm;
use std::time::{Duration, Instant};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub enum Event {
    Tick,
    Key(crossterm::event::KeyEvent),
}

pub struct EventHandler {
    rx: std::sync::mpsc::Receiver<Event>,
}

impl EventHandler {
    pub fn new() -> Self {
        const POLL_TIME: Duration = Duration::from_millis(16);
        const TICK_INTERVAL: Duration = Duration::from_millis(500);

        let (tx, rx) = std::sync::mpsc::channel();
        let mut now = Instant::now();
        let mut last_event = Instant::now();
        std::thread::spawn(move || loop {
            if crossterm::event::poll(POLL_TIME).expect("event poll failed") {
                match crossterm::event::read().expect("event read failed") {
                    crossterm::event::Event::Key(e) => {
                        last_event = Instant::now();
                        tx.send(Event::Key(e))
                    }
                    crossterm::event::Event::Resize(_, _) => Ok(()),
                    _ => unimplemented!(),
                }
                .expect("event send failed")
            }
            // only tick when idle.
            let time_since_last_event: Duration = Instant::now() - last_event;
            if now.elapsed() >= TICK_INTERVAL && time_since_last_event >= TICK_INTERVAL
            {
                tx.send(Event::Tick).expect("tick send failed");
                now = Instant::now();
            }
        });
        EventHandler { rx }
    }

    pub fn next(&self) -> Result<Event> {
        Ok(self.rx.recv()?)
    }
}
