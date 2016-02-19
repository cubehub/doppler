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


// import local modules
extern crate doppler;
use doppler::usage;
use doppler::usage::Mode::{ConstMode, TrackMode};
use doppler::usage::DataType::{I16, F32};
use doppler::dsp;

// import external modules
#[macro_use]
extern crate log;
extern crate fern;
use std::process::exit;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::slice;

extern crate time;
extern crate gpredict;
use gpredict::{Predict, Tle, Location};

const SPEED_OF_LIGHT_M_S: f64 = 299792458.;
const BUFFER_SIZE: usize = 8192;

fn main() {
    setup_logger();
    let args = usage::args();

    info!("doppler {} andres.vahter@gmail.com\n\n", env!("CARGO_PKG_VERSION"));

    let mut stdin = BufReader::with_capacity(BUFFER_SIZE*2, io::stdin());
    let mut stdout = BufWriter::new(io::stdout());

    let mut samplenr: u64 = 0;
    let mut samplenr_ofs: u64 = 0;

    let mut shift = |intype: doppler::usage::DataType, shift_hz: f64, samplerate: u32| {
        let invec = stdin.by_ref().bytes().take(BUFFER_SIZE).collect::<Result<Vec<u8>,_>>().ok().expect("doppler collect error");

        let input = match intype {
                I16 => dsp::convert_iqi16_to_complex(&invec),
                F32 => dsp::convert_iqf32_to_complex(&invec),
        };

        let output = dsp::shift_frequency(&input, &mut samplenr, &mut samplenr_ofs, shift_hz, samplerate);

        match *args.outputtype.as_ref().unwrap() {
            doppler::usage::DataType::I16 => {
                let mut outputi16 = Vec::<u8>::with_capacity(output.len() * 4);

                for sample in &output[..] {
                    let i = (sample.re * 32767.0) as i16;
                    let q = (sample.im * 32767.0) as i16;

                    outputi16.push((i & 0xFF) as u8);
                    outputi16.push(((i >> 8) & 0xFF) as u8);
                    outputi16.push((q & 0xFF) as u8);
                    outputi16.push(((q >> 8) & 0xFF) as u8);
                }

                stdout.write(&outputi16[..]).map_err(|e|{info!("doppler stdout.write error: {:?}", e)}).unwrap();
            },

            doppler::usage::DataType::F32 => {
                // * 8 because Complex<f32> is 8 bytes long
                let slice = unsafe {slice::from_raw_parts(output.as_ptr() as *const _, (output.len() * 8))};
                stdout.write(&slice).map_err(|e|{info!("doppler stdout.write error: {:?}", e)}).unwrap();
            },
        };


        stdout.flush().map_err(|e|{info!("doppler stdout.flush error: {:?}", e)}).unwrap();
        (invec.len() != BUFFER_SIZE, output.len())
    };

    match *args.mode.as_ref().unwrap() {
        ConstMode => {
            info!("constant shift mode");
            info!("\tIQ samplerate   : {}", args.samplerate.as_ref().unwrap());
            info!("\tIQ input type   : {}", args.inputtype.as_ref().unwrap());
            info!("\tIQ output type  : {}\n", args.outputtype.as_ref().unwrap());
            info!("\tfrequency shift : {} Hz", args.constargs.shift.as_ref().unwrap());

            let intype = args.inputtype.unwrap();
            let shift_hz = args.constargs.shift.unwrap() as f64;
            let samplerate = args.samplerate.unwrap();

            loop {
                let stop_and_count: (bool, usize) = shift(intype, shift_hz, samplerate);
                if stop_and_count.0 {
                    break;
                }
            }
        }


        TrackMode => {
            info!("tracking mode");
            info!("\tIQ samplerate   : {}", args.samplerate.as_ref().unwrap());
            info!("\tIQ input type   : {}", args.inputtype.as_ref().unwrap());
            info!("\tIQ output type  : {}\n", args.outputtype.as_ref().unwrap());
            info!("\tTLE file        : {}", args.trackargs.tlefile.as_ref().unwrap());
            info!("\tTLE name        : {}", args.trackargs.tlename.as_ref().unwrap());
            info!("\tlocation        : {:?}", args.trackargs.location.as_ref().unwrap());
            if args.trackargs.time.is_some() {
                info!("\ttime            : {:.3}", args.trackargs.time.unwrap().to_utc().rfc3339());
            }
            info!("\tfrequency       : {} Hz", args.trackargs.frequency.as_ref().unwrap());
            info!("\toffset          : {} Hz\n\n\n", args.trackargs.offset.unwrap_or(0));

            let l = args.trackargs.location.unwrap();
            let location: Location = Location{lat_deg: l.lat, lon_deg: l.lon, alt_m: l.alt};
            let tlename = args.trackargs.tlename.as_ref().unwrap();
            let tlefile = args.trackargs.tlefile.as_ref().unwrap();

            let tle = match Tle::from_file(&tlename, &tlefile) {
                Ok(t) => {t},
                Err(e) => {
                    info!("{}", e);
                    exit(1);
                }
            };

            let mut predict: Predict = Predict::new(&tle, &location);
            let intype = args.inputtype.unwrap();

            let samplerate = args.samplerate.unwrap();
            let mut last_time: time::Tm = time::now_utc();

            match args.trackargs.time {
                Some(start_time) => {
                    let mut sample_count = 0;
                    let mut dt = time::Duration::seconds(0);
                    last_time = start_time;

                    loop {
                        predict.update(Some(start_time + dt));
                        let doppler_hz = (predict.sat.range_rate_km_sec * 1000 as f64 / SPEED_OF_LIGHT_M_S as f64) * args.trackargs.frequency.unwrap() as f64 * (-1.0);

                        // advance time based on how many samples are read in
                        dt = time::Duration::seconds((sample_count as f32 / samplerate as f32) as i64);
                        if start_time + dt - last_time >= time::Duration::seconds(5) {
                            last_time = start_time + dt;
                            info!("time                : {:}", (start_time + dt).to_utc().rfc3339());
                            info!("az                  : {:.2}°", predict.sat.az_deg);
                            info!("el                  : {:.2}°", predict.sat.el_deg);
                            info!("range               : {:.0} km", predict.sat.range_km);
                            info!("range rate          : {:.3} km/sec", predict.sat.range_rate_km_sec);
                            info!("doppler@{:.3} MHz : {:.2} Hz\n", args.trackargs.frequency.unwrap() as f64 / 1000_000_f64, doppler_hz);
                        }

                        let (stop, count): (bool, usize) = shift(intype, doppler_hz + args.trackargs.offset.unwrap_or(0) as f64, samplerate);
                        if stop {
                            break;
                        }

                        sample_count += count;
                    }
                }

                None => {
                    loop {
                        predict.update(None);
                        let doppler_hz = (predict.sat.range_rate_km_sec * 1000 as f64 / SPEED_OF_LIGHT_M_S as f64) * args.trackargs.frequency.unwrap() as f64 * (-1.0);

                        if time::now_utc() - last_time >= time::Duration::seconds(1) {
                            last_time = time::now_utc();
                            info!("time                : {:}", time::now_utc().to_utc().rfc3339());
                            info!("az                  : {:.2}°", predict.sat.az_deg);
                            info!("el                  : {:.2}°", predict.sat.el_deg);
                            info!("range               : {:.0} km", predict.sat.range_km);
                            info!("range rate          : {:.3} km/sec", predict.sat.range_rate_km_sec);
                            info!("doppler@{:.3} MHz : {:.2} Hz\n", args.trackargs.frequency.unwrap() as f64 / 1000_000_f64, doppler_hz);
                        }

                        let (stop, _): (bool, usize) = shift(intype, doppler_hz + args.trackargs.offset.unwrap_or(0) as f64, samplerate);
                        if stop {
                            break;
                        }
                    }
                }
            };
        }
    }
}

fn setup_logger() {
    let logger_config = fern::DispatchConfig {
        format: Box::new(|msg: &str, level: &log::LogLevel, _location: &log::LogLocation| {
            let t = time::now();
            let ms = t.tm_nsec/1000_000;
            let path = _location.__module_path;
            let line = _location.__line;

            format!("{}.{:3} [{:<6} {:<30} {:>3}]  {}",
                    t.strftime("%Y-%m-%dT%H:%M:%S")
                     .unwrap_or_else(|err| panic!("strftime format error: {}", err)),
                    ms, level, path, line, msg)
        }),
        output: vec![fern::OutputConfig::stderr()],
        level: log::LogLevelFilter::Debug,
        directives: vec!()
    };

    if let Err(e) = fern::init_global_logger(logger_config, log::LogLevelFilter::Trace) {
        panic!("Failed to initialize global logger: {}", e);
    }
}
