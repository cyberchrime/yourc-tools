use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use midir::{MidiInput, MidiOutput, MidiInputConnection, MidiOutputConnection};

use crate::device::midi::Midi;

pub struct Sysex {
    midi: Midi,
}

#[cfg(not(any(windows, target_arch = "wasm32")))] // virtual ports are not supported on Windows nor on Web MIDI
impl Sysex {
    const LARGE_SYSEX_SIZE: usize = 100;

    pub fn send(&mut self, data: &[u8]) {
        let buffer: Vec<u8> = [&[0xf0], data, &[0xf7]].concat();
        self.midi.send(&buffer);
    }

    pub fn new(midi: Midi) -> Result<Sysex, Box<dyn Error>> {
        Ok(Sysex {
            midi,
        })
    }
}