use crossterm::event::{self, Event as CrosstermEvent, KeyEvent};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug)]
pub(crate) enum Event {
    Tick,
    Key(KeyEvent),
    // Mouse(MouseEvent),
    // Resize(u16, u16),
}

#[allow(dead_code)]
#[derive(Debug)]
pub(crate) struct EventHandler {
    sender: mpsc::Sender<Event>,
    receiver: mpsc::Receiver<Event>,
    handler: thread::JoinHandle<()>,
}

impl EventHandler {
    pub fn new(tick_rate: u64) -> Self {
        let tick_rate = Duration::from_millis(tick_rate);
        let (sender, receiver) = mpsc::channel();
        let handler = {
            let sender = sender.clone();
            thread::spawn(move || {
                let mut last_tick = Instant::now();
                loop {
                    let timeout = tick_rate
                        .checked_sub(last_tick.elapsed())
                        .unwrap_or(tick_rate);

                    if event::poll(timeout).expect("failed to poll new events") {
                        match event::read().expect("unable to read event") {
                            CrosstermEvent::Key(e) => sender.send(Event::Key(e)),
                            CrosstermEvent::Mouse(_e) => Ok(()), // sender.send(Event::Mouse(e)),
                            CrosstermEvent::Resize(_w, _h) => Ok(()), //sender.send(Event::Resize(w, h)),
                            CrosstermEvent::FocusGained => Ok(()),
                            CrosstermEvent::FocusLost => Ok(()),
                            CrosstermEvent::Paste(_) => unimplemented!(),
                        }
                        .expect("failed to send terminal event")
                    }

                    if last_tick.elapsed() >= tick_rate {
                        sender.send(Event::Tick).expect("failed to send tick event");
                        last_tick = Instant::now();
                    }
                }
            })
        };
        Self {
            sender,
            receiver,
            handler,
        }
    }

    pub fn next(&self) -> color_eyre::Result<Event> {
        Ok(self.receiver.recv()?)
    }
}
