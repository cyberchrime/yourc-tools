use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

use std::io::Write;

use midir::{MidiInput, MidiOutput, MidiInputConnection, MidiOutputConnection};

pub struct Midi {
    input: MidiInputConnection<()>,
    output: MidiOutputConnection,
}

#[cfg(not(any(windows, target_arch = "wasm32")))] // virtual ports are not supported on Windows nor on Web MIDI
impl Midi {
    const MIDI_NAME: &str = "yoUR-C";
    const LARGE_SYSEX_SIZE: usize = 100; // This is the maximum that worked for me

    pub fn send(&mut self, data: &[u8]) -> Result<(), Box<dyn Error>> {
        self.output.send(data)?;
        Ok(())
    }

    fn connect_output(port_name: &str) -> Result<MidiOutputConnection, Box<dyn Error>> {
        let midi_out = MidiOutput::new(Midi::MIDI_NAME)?;
        let out_ports = midi_out.ports();
        let ur44c_ports = out_ports.iter().filter(|p| {
            midi_out.port_name(&p).unwrap().starts_with(port_name)
        });

        if let Some(sysex_port) = ur44c_ports.last() {
            let conn_out = midi_out.connect(&sysex_port, "Out")?;
            Ok(conn_out)
        } else {
            Err(format!("Port \"{}\" not found.", port_name))?
        }
    }

    fn connect_input(port_name: &str) -> Result<MidiInputConnection<()>, Box<dyn Error>> {
        let midi_in = MidiInput::new(Midi::MIDI_NAME)?;
        let in_ports = midi_in.ports();
        let ur44c_ports = in_ports.iter().filter(|p| {
            midi_in.port_name(&p).unwrap().starts_with(port_name)
        });

        if let Some(sysex_port) = ur44c_ports.last() {
            let conn_in = midi_in.connect(&sysex_port, "In", |size, data, _| println!("{size} {:?}", data), ())?;
            Ok(conn_in)
        } else {
            Err(format!("Port \"{}\" not found.", port_name))?
        }
    }

    pub fn new(port_name: &str) -> Result<Midi, Box<dyn Error>> {
        let conn_out = Midi::connect_output(port_name)?;
        let conn_in = Midi::connect_input(port_name)?;

        let time = Duration::new(5, 0);
        sleep(time);

        Ok(Midi {
            input: conn_in,
            output: conn_out,
        })
    }
}