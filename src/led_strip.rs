use rs_ws281x::{ChannelBuilder, Controller, ControllerBuilder, StripType};

use std::ops::{Index, IndexMut};

use crate::{colors::ColorRGB, Error};

pub struct LedStrip {
    controller: Controller,
    buffer: Vec<ColorRGB>,
}

impl LedStrip {
    pub fn new(pin: u32, length: u32) -> Result<Self, Error> {
        let channel = ChannelBuilder::new()
            .pin(pin as i32)
            .count(length as i32)
            .brightness(255)
            .strip_type(StripType::Ws2812)
            .build();

        let controller = ControllerBuilder::new().channel(0, channel).build()?;
        let buffer = vec![(0, 0, 0); length as usize];

        Ok(Self { controller, buffer })
    }

    pub fn render(&mut self) -> Result<(), Error> {
        let leds = self.controller.leds_mut(0);

        for (i, &(r, g, b)) in self.buffer.iter().enumerate() {
            leds[i] = [b, g, r, 0];
        }

        self.controller.render()?;
        Ok(())
    }

    pub fn write(&mut self, pixels: &[ColorRGB]) -> Result<(), Error> {
        self.buffer.copy_from_slice(pixels);
        self.render()
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }
}

impl Index<usize> for LedStrip {
    type Output = ColorRGB;
    fn index(&self, index: usize) -> &ColorRGB {
        &self.buffer[index]
    }
}

impl IndexMut<usize> for LedStrip {
    fn index_mut(&mut self, index: usize) -> &mut ColorRGB {
        &mut self.buffer[index]
    }
}
