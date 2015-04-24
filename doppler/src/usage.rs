/*
 * The MIT License (MIT)
 *
 * Copyright (c) 2015 Andres Vahter (andres.vahter@gmail.com)
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use clap::{App, Arg, SubCommand};
use self::InputType::{F32, I16};
use self::Mode::{ConstMode, TrackMode};

use std::fmt;

/*
static USAGE: &'static str = "
doppler <andres.vahter@gmail.com>

Usage:
    doppler (const (--samplerate <sps> | -s <sps>) --intype <type> --shift <Hz>)
    doppler (track (--samplerate <sps> | -s <sps>) --intype <type> --tlefile <file> --tlename <name> --location <lat,lon,alt> --freq <Hz>) [--time <Y-m-dTH:M:S>] [--shift <Hz>]
    doppler (-h | --help | --version)

Options:
    -s --samplerate <sps>       IQ data samplerate.
    --intype <type>             IQ data type <i16, f32>.

    -h --help                   Show this screen.
    --version                   Show version.

Const mode options:
    --shift <Hz>                Constant frequency shift in Hz [default: 0].

Track mode options:
    --tlefile <file>            TLE database file eg. \"http://www.celestrak.com/NORAD/elements/cubesat.txt\".
    --tlename <name>            TLE name eg. 'ESTCUBE 1'.
    --location <lat,lon,alt>    Observer location on earth.
    --time <Y-m-dTH:M:S>        Observation start time. It should be specified if input is IQ data recording. Real time is used otherwise.
    --freq <Hz>                 Satellite transmitter frequency in Hz.
    --shift <Hz>                Constant frequency shift in Hz [default: 0].
";

*/

pub enum Mode {
    ConstMode,
    TrackMode,
}

pub enum InputType {
    F32,
    I16,
}

impl fmt::Display for InputType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InputType::F32 => {write!(f, "f32")},
            InputType::I16 => {write!(f, "i16")},
        }
    }
}

pub struct ConstModeArgs {
    pub shift: Option<u32>,
}

pub struct TrackModeArgs {
    pub tlefile: Option<String>,
    pub tlename: Option<String>,
    pub location: Option<String>,
    pub time: Option<u32>,
    pub frequency: Option<u32>,
    pub shift: Option<u32>,
}

pub struct CommandArgs {
    pub mode: Option<Mode>,

    pub samplerate: Option<u32>,
    pub inputtype: Option<InputType>,

    pub constargs: ConstModeArgs,
    pub trackargs: TrackModeArgs,
}

pub fn args() -> CommandArgs {
    let matches = App::new("doppler")
                .author("Andres Vahter <andres.vahter@gmail.com>")
                .version(env!("CARGO_PKG_VERSION"))
                .about("Compensates IQ data stream doppler shift based on TLE information and constant shift for IQ data is also possible.")


                .subcommand(SubCommand::new("const")
                    .about("Constant shift mode")

                    .arg(Arg::with_name("SAMPLERATE")
                       .long("samplerate")
                       .short("s")
                       .help("IQ data samplerate")
                       .required(true)
                       .takes_value(true))

                    .arg(Arg::with_name("INTYPE")
                       .long("intype")
                       .short("i")
                       .help("IQ data type")
                       .required(true)
                       .possible_values(vec!["i16", "f32"])
                       .takes_value(true))

                    .arg(Arg::with_name("SHIFT")
                       .long("shift")
                       .help("frequency shift in Hz")
                       .required(true)
                       .takes_value(true)))


                .subcommand(SubCommand::new("track")
                    .about("Doppler tracking mode")

                    .arg(Arg::with_name("SAMPLERATE")
                       .long("samplerate")
                       .short("s")
                       .help("IQ data samplerate")
                       .required(true)
                       .takes_value(true))

                    .arg(Arg::with_name("INTYPE")
                       .long("intype")
                       .short("i")
                       .help("IQ data type")
                       .required(true)
                       .possible_values(vec!["i16", "f32"])
                       .takes_value(true))

                    .arg(Arg::with_name("TLEFILE")
                       .long("tlefile")
                       .help("TLE file: eg. http://www.celestrak.com/NORAD/elements/cubesat.txt")
                       .required(true)
                       .takes_value(true)))

                .get_matches();


    let mut args = CommandArgs {
                    mode : None,

                    samplerate : None,
                    inputtype : None,

                    constargs : ConstModeArgs {
                        shift: None,
                    },

                    trackargs : TrackModeArgs {
                        tlefile : None,
                        tlename : None,
                        location: None,
                        time : None,
                        frequency : None,
                        shift : None,
                    },
                };


    match matches.subcommand_name() {
        Some("const")   => {
            args.mode = Some(ConstMode);
            let submatches = matches.subcommand_matches("const").unwrap();
            args.samplerate = Some(value_t_or_exit!(submatches.value_of("SAMPLERATE"), u32));

            match submatches.value_of("INTYPE").unwrap() {
                "f32" => {args.inputtype = Some(F32);},
                "i16" => {args.inputtype = Some(I16);},
                _ => unreachable!()
            }

            args.constargs.shift = Some(value_t_or_exit!(submatches.value_of("SHIFT"), u32));
        },

        Some("track") => {
            args.mode = Some(TrackMode);
        },

        _ => unreachable!()
    }

    args
}

