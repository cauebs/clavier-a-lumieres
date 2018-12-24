use wmidi::MidiMessage;

use crate::colors::{ColorRGB, CHROMATIC_RAINBOW, RAINBOW_OF_FIFTHS};

use std::cmp::max;

const NUM_KEYS: usize = 88;

pub struct Piano {
    keys: [u8; NUM_KEYS],
    strings: [u32; NUM_KEYS],
    soft_pedal: bool,
    sostenuto_pedal: [bool; NUM_KEYS],
    sustaining_pedal: u8,
}

impl Piano {
    pub fn new() -> Self {
        Self {
            keys: [0; NUM_KEYS],
            strings: [0; NUM_KEYS],
            soft_pedal: false,
            sostenuto_pedal: [false; NUM_KEYS],
            sustaining_pedal: 0,
        }
    }

    pub fn step(&mut self) {
        for (i, s) in self.strings.iter_mut().enumerate() {
            let decay = {
                if self.keys[i] > 0 || self.sustaining_pedal > 64 || self.sostenuto_pedal[i] {
                    f64::from(*s) / 100.0
                } else {
                    f64::from(*s) / 5.0
                }
            };

            *s = s.saturating_sub(decay.round() as u32);
        }
    }

    pub fn update(&mut self, message: MidiMessage) {
        match message {
            MidiMessage::NoteOn(_, note, velocity) => {
                let note = (note - 21) as usize;
                self.keys[note] = velocity;
                self.strings[note] = max(self.strings[note], u32::from(velocity) << 24);
            }

            MidiMessage::NoteOff(_, note, _) => {
                let note = (note - 21) as usize;
                self.keys[note] = 0;
                // if self.sustaining_pedal == 0 && !self.sostenuto_pedal[note] {
                //     self.strings[note] = 0;
                // }
            }

            MidiMessage::ControlChange(_, number, value) => match number {
                67 => self.soft_pedal = value != 0,
                66 => {
                    if value == 0 {
                        self.sostenuto_pedal.copy_from_slice(&[false; NUM_KEYS]);
                    } else {
                        for (i, &key) in self.keys.iter().enumerate() {
                            if key > 0 {
                                self.sostenuto_pedal[i] = true;
                            }
                        }
                    }
                }
                64 => self.sustaining_pedal = value,
                _ => {}
            },
            _ => {}
        }
    }

    pub fn to_leds(&self, length: usize) -> Vec<ColorRGB> {
        let mut leds = vec![(0, 0, 0); length];

        for (i, &value) in self.strings.iter().enumerate() {
            if value == 0 {
                continue;
            }

            let value = u64::from(value);
            let value = ((value * value) >> 54) as f64 / 252.0;

            let light_pos = i as f64 / 88.0 * 59.0;

            let left_led_index = light_pos.floor() as usize;
            let right_led_index = light_pos.ceil() as usize;

            let mut paint_led = |index: usize| {
                let mut left_led_intensity = 1.0;
                left_led_intensity -= (light_pos - index as f64).abs();
                left_led_intensity *= value;

                let (r, g, b) = RAINBOW_OF_FIFTHS[i % 12];
                let r = (f64::from(r) * left_led_intensity) as u8;
                let g = (f64::from(g) * left_led_intensity) as u8;
                let b = (f64::from(b) * left_led_intensity) as u8;

                let left_led: &mut ColorRGB = &mut leds[index as usize];
                left_led.0 = left_led.0.saturating_add(r);
                left_led.1 = left_led.1.saturating_add(g);
                left_led.2 = left_led.2.saturating_add(b);
            };

            paint_led(left_led_index);
            paint_led(right_led_index);
        }

        leds
    }
}
