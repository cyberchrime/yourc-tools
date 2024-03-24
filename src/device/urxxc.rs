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

    #[allow(non_snake_case)]
    fn encode_4B5(data: i32) -> [u8; 5] {
        let mut encoded: [u8; 5] = [0; 5];
        
        encoded[0] = (0x7f & (data >> 7*4)) as u8;
        encoded[1] = (0x7f & (data >> 7*3)) as u8;
        encoded[2] = (0x7f & (data >> 7*2)) as u8;
        encoded[3] = (0x7f & (data >> 7*1)) as u8;
        encoded[4] = (0x7f & data) as u8;
        
        encoded
    }

    fn encode_14b16(data: u16) -> [u8; 2] {
        let mut encoded: [u8; 2] = [0; 2];
        
        encoded[0] = (0x7f & (data >> 7)) as u8;
        encoded[1] = (0x7f & data) as u8;
        
        encoded
    }

    pub fn set(&mut self, param: u16, channel: u8, value: i32) {
        let mut data = vec![0x43, 0x10, 0x3E, 0x14, 0x01, 0x01, 0x00];
        data.extend_from_slice(&Self::encode_14b16(param));
        data.extend_from_slice(&[0x00, 0x00]);
        data.extend_from_slice(&[channel]);
        data.extend_from_slice(&Self::encode_4B5(value));
        self.sysex.send(&data);
    }

    pub fn request(&mut self, param: u16, channel: u8) {
        let mut data = vec![0x43, 0x30, 0x3E, 0x14, 0x01, 0x04, 0x02, 0x00];
        data.extend_from_slice(&Self::encode_14b16(param));
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