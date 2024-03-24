use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use midir::{MidiInput, MidiOutput, MidiInputConnection, MidiOutputConnection};

use crate::device::sysex::Sysex;

pub struct URxxC {
    sysex: Sysex,
}

#[cfg(not(any(windows, target_arch = "wasm32")))] // virtual ports are not supported on Windows nor on Web MIDI
impl URxxC {
    const LARGE_SYSEX_SIZE: usize = 100;

    pub fn set(&mut self, param: u16, channel: u8, value: u32) {
        let mut data = vec![0x43, 0x10, 0x3E, 0x14, 0x01, 0x01, 0x00];
        data.extend_from_slice(&param.to_be_bytes());
        data.extend_from_slice(&[0x00, 0x00]);
        data.extend_from_slice(&[channel]);
        data.extend_from_slice(&value.to_be_bytes());
        self.sysex.send(&data);
    }

    pub fn request(&mut self, param: u8, channel: u8) {
        let mut data = vec![0x43, 0x30, 0x3E, 0x14, 0x01, 0x04, 0x02, 0x00];
        data.extend_from_slice(&param.to_be_bytes());
        data.extend_from_slice(&[0x00, 0x00]);
        data.extend_from_slice(&[channel]);
        self.sysex.send(&data);
    }

    pub fn new(sysex: Sysex) -> Result<URxxC, Box<dyn Error>> {
        Ok(URxxC {
            sysex,
        })
    }
}