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
use time;
use self::DataType::{F32, I16};
use self::Mode::{ConstMode, TrackMode};

use std::fmt;
use std::process::exit;

pub enum Mode {
    ConstMode,
    TrackMode,
}

#[derive(Clone, Copy)]
pub enum DataType {
    F32,
    I16,
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DataType::F32 => {write!(f, "f32")},
            DataType::I16 => {write!(f, "i16")},
        }
    }
}

#[derive(Debug)]
#[derive(Clone, Copy)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
    pub alt: i32,
}

pub struct ConstModeArgs {
    pub shift: Option<i32>,
}

pub struct TrackModeArgs {
    pub tlefile: Option<String>,
    pub tlename: Option<String>,
    pub location: Option<Location>,
    pub time: Option<time::Tm>,
    pub frequency: Option<u32>,
    pub offset: Option<i32>,
}

pub struct CommandArgs {
    pub mode: Option<Mode>,

    pub samplerate: Option<u32>,
    pub inputtype: Option<DataType>,
    pub outputtype: Option<DataType>,

    pub constargs: ConstModeArgs,
    pub trackargs: TrackModeArgs,
}

fn parse_location(location: &str) -> Result<Location, String> {
    if location.contains("lat") && location.contains("lon") && location.contains("alt"){
        let split = location.split(",");

        let mut lat: Option<f64> = None;
        let mut lon: Option<f64> = None;
        let mut alt: Option<i32> = None;

        for s in split {
            if s.contains("lat") && s.contains("=") {
                lat = s.split("=").nth(1).unwrap().parse::<f64>().ok();
            }
            else if s.contains("lon") && s.contains("=") {
                lon = s.split("=").nth(1).unwrap().parse::<f64>().ok();
            }
            else if s.contains("alt") && s.contains("=") {
                alt = s.split("=").nth(1).unwrap().parse::<i32>().ok();
            }
        }

        if lat.is_some() && lon.is_some() && alt.is_some() {
            Ok(Location{lat: lat.unwrap(), lon: lon.unwrap(), alt: alt.unwrap()})
        }
        else {
            Err(format!("{} isn't a valid value for --location\n\t[use as: lat=58.64560,lon=23.15163,alt=8]", location).to_string())
        }
    }
    else {
        Err("--location should be defined as: lat=58.64560,lon=23.15163,alt=8".to_string())
    }
}

pub fn args() -> CommandArgs {
    let datatypes = ["i16", "f32"];

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
                       .help("IQ data input type")
                       .required(true)
                       .possible_values(&datatypes)
                       .takes_value(true))

                   .arg(Arg::with_name("OUTTYPE")
                       .long("outtype")
                       .short("o")
                       .help("IQ data output type")
                       .required(false)
                       .possible_values(&datatypes)
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
                       .possible_values(&datatypes)
                       .takes_value(true))

                   .arg(Arg::with_name("OUTTYPE")
                       .long("outtype")
                       .short("o")
                       .help("IQ data output type")
                       .required(false)
                       .possible_values(&datatypes)
                       .takes_value(true))

                    .arg(Arg::with_name("TLEFILE")
                       .long("tlefile")
                       .help("TLE file: eg. http://www.celestrak.com/NORAD/elements/cubesat.txt")
                       .required(true)
                       .takes_value(true))

                    .arg(Arg::with_name("TLENAME")
                       .long("tlename")
                       .help("TLE name in TLE file: eg. ESTCUBE 1")
                       .required(true)
                       .takes_value(true))

                    .arg(Arg::with_name("LOCATION")
                       .long("location")
                       .help("Observer location: eg. lat=58.64560,lon=23.15163,alt=8")
                       .required(true)
                       .takes_value(true))

                    .arg(Arg::with_name("TIME")
                       .long("time")
                       .help("Observation start time in UTC Y-m-dTH:M:S: eg. 2015-05-13T14:28:48. If not specified current time is used")
                       .required(false)
                       .takes_value(true))

                    .arg(Arg::with_name("FREQUENCY")
                       .long("frequency")
                       .help("Satellite transmitter frequency in Hz")
                       .required(true)
                       .takes_value(true))

                    .arg(Arg::with_name("OFFSET")
                       .long("offset")
                       .help("Constant frequency shift in Hz. Can be used to compensate constant offset")
                       .required(false)
                       .takes_value(true)))

                .get_matches();


    let mut args = CommandArgs {
                    mode : None,

                    samplerate : None,
                    inputtype : None,
                    outputtype: None,

                    constargs : ConstModeArgs {
                        shift: None,
                    },

                    trackargs : TrackModeArgs {
                        tlefile : None,
                        tlename : None,
                        location: None,
                        time : None,
                        frequency : None,
                        offset : None,
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

            if submatches.is_present("OUTTYPE") {
                match submatches.value_of("OUTTYPE").unwrap() {
                    "f32" => {args.outputtype = Some(F32);},
                    "i16" => {args.outputtype = Some(I16);},
                    _ => unreachable!()
                }
            }
            else {
                args.outputtype = args.inputtype;
            }

            args.constargs.shift = Some(value_t_or_exit!(submatches.value_of("SHIFT"), i32));
        },


        Some("track") => {
            args.mode = Some(TrackMode);
            let submatches = matches.subcommand_matches("track").unwrap();
            args.samplerate = Some(value_t_or_exit!(submatches.value_of("SAMPLERATE"), u32));

            match submatches.value_of("INTYPE").unwrap() {
                "f32" => {args.inputtype = Some(F32);},
                "i16" => {args.inputtype = Some(I16);},
                _ => unreachable!()
            }

            if submatches.is_present("OUTTYPE") {
                match submatches.value_of("OUTTYPE").unwrap() {
                    "f32" => {args.outputtype = Some(F32);},
                    "i16" => {args.outputtype = Some(I16);},
                    _ => unreachable!()
                }
            }
            else {
                args.outputtype = args.inputtype;
            }

            if submatches.is_present("OFFSET") {
                args.trackargs.offset = Some(value_t_or_exit!(submatches.value_of("OFFSET"), i32));
            }

            if submatches.is_present("TIME") {
                let tm = time::strptime(submatches.value_of("TIME").unwrap(), "%Y-%m-%dT%H:%M:%S");
                match tm {
                    Ok(_) => {},
                    Err(e) => {
                        println!("{}.", e);
                        println!("--time should be defined in Y-m-dTH:M:S format: eg. 2015-05-13T14:28:48");
                        exit(1);
                    },
                };

                args.trackargs.time = Some(tm.unwrap());
            }

            args.trackargs.tlefile = Some(submatches.value_of("TLEFILE").unwrap().to_string());
            args.trackargs.tlename = Some(submatches.value_of("TLENAME").unwrap().to_string());
            args.trackargs.frequency = Some(value_t_or_exit!(submatches.value_of("FREQUENCY"), u32));

            let location = parse_location(&submatches.value_of("LOCATION").unwrap().to_string());
            match location {
                Ok(loc) => { args.trackargs.location = Some(loc);},
                Err(e) => {
                    println!("{}.", e);
                    exit(1);
                }
            }
        },

        _ => unreachable!()
    }

    args
}
