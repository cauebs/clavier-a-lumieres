use midir::{Ignore, MidiInput};
use wmidi::MidiMessage;

use std::{
    sync::{Arc, Mutex},
    thread::sleep,
    time::Duration,
};

mod colors;

mod led_strip;
use crate::led_strip::LedStrip;

mod piano;
use crate::piano::Piano;

pub type Error = Box<dyn std::error::Error>;

fn update(_timestamp: u64, message: &[u8], piano: &mut Arc<Mutex<Piano>>) {
    let message = MidiMessage::from_bytes(message).expect("Invalid message.");
    piano
        .lock()
        .expect("Piano's mutex was poisoned!")
        .update(message);
}

fn main() -> Result<(), Error> {
    let piano = Arc::new(Mutex::new(Piano::new()));

    let mut midi = MidiInput::new("piano")?;
    midi.ignore(Ignore::ActiveSense);
    let _connection = midi.connect(1, "piano", update, Arc::clone(&piano))?;

    let mut led_strip = LedStrip::new(18, 60)?;
    let delay = Duration::from_millis(20);

    loop {
        sleep(delay);

        let mut piano = piano.lock().unwrap();
        piano.step();

        led_strip.write(&piano.to_leds(led_strip.len()))?;
    }
}
