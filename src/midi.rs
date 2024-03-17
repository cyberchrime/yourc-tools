use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use midir::{MidiInput, MidiOutput, MidiInputConnection, MidiOutputConnection};

pub struct Midi {
    input: MidiInputConnection<()>,
    output: MidiOutputConnection,
}

#[cfg(not(any(windows, target_arch = "wasm32")))] // virtual ports are not supported on Windows nor on Web MIDI
impl Midi {
    const MIDI_NAME: &str = "yoUR-C";
    const LARGE_SYSEX_SIZE: usize = 100; // This is the maximum that worked for me

    fn connect_output() -> Result<MidiOutputConnection, Box<dyn Error>> {
        let midi_out = MidiOutput::new(Midi::MIDI_NAME)?;
        let out_ports = midi_out.ports();
        let ur44c_ports = out_ports.iter().filter(|p| {
            midi_out.port_name(&p).unwrap().starts_with("Steinberg UR44C:Steinberg UR44C MIDI")
        });

        if let Some(sysex_port) = ur44c_ports.last() {
            let mut conn_out = midi_out.connect(&sysex_port, "Out")?;
            Ok(conn_out)
        } else {
            Err("No UR44C MIDI port found")?
        }
    }

    fn connect_input() -> Result<MidiInputConnection<()>, Box<dyn Error>> {
        let midi_in = MidiInput::new(Midi::MIDI_NAME)?;
        let in_ports = midi_in.ports();
        let ur44c_ports = in_ports.iter().filter(|p| {
            midi_in.port_name(&p).unwrap().starts_with("Steinberg UR44C:Steinberg UR44C MIDI")
        });

        if let Some(sysex_port) = ur44c_ports.last() {
            let mut conn_in = midi_in.connect(&sysex_port, "In", |size, data, _| println!("{size} {:?}", data), ())?;
            Ok(conn_in)
        } else {
            Err("No UR44C MIDI port found")?
        }
    }

    pub fn new() -> Result<Midi, Box<dyn Error>> {
        let conn_out = Midi::connect_output()?;
        let conn_in = Midi::connect_input()?;

        let time = Duration::new(5, 0);
        sleep(time);

        Ok(Midi {
            input: conn_in,
            output: conn_out,
        })
    }
}