/* yoUR-C Tools
 * Copyright (C) 2024 Chris H. Meyer
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 * 
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 * 
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

mod device;

use std::{thread::sleep, time::Duration};

use crate::device::midi::Midi;
use crate::device::sysex::Sysex;
use crate::device::urxxc::URxxC;


use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Set a parameter
    Write(WriteCommand),
    /// Read a parameter
    Read(ReadCommand),
    /// Dump the whole configuration
    Dump { filename: String},
    /// Set a previously dumped configuration
    Restore { filename: String},
}

#[derive(Debug, Args)]
struct WriteCommand {
    parameter: u16,
    channel: u8,
    value: i32,
}

#[derive(Debug, Args)]
struct ReadCommand {
    parameter: u16,
    channel: u8,
}

fn main() {
    let args = Cli::parse();

    let midi = Midi::new("Steinberg UR44C:Steinberg UR44C MIDI").unwrap();
    let sysex: Sysex = Sysex::new(midi).unwrap();
    let mut urxxc: URxxC = URxxC::new(sysex).unwrap();

    match &args.command {
        Some(Commands::Write(cmd)) => {
            urxxc.set(cmd.parameter, cmd.channel, cmd.value);
            println!("Write: {:?}", cmd);
        }
        Some(Commands::Read(cmd)) => {
            urxxc.request(cmd.parameter, cmd.channel);
            println!("Read: {:?}", cmd);

            let d = Duration::from_secs(1);
            sleep(d);
        }
        Some(Commands::Dump { filename }) => {
            println!("Dump: {}", filename);
        }
        Some(Commands::Restore { filename }) => {
            println!("Restore: {}", filename);
        }
        None => println!("GUI not yet implemented.")
    }

}