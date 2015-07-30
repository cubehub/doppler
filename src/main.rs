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
use doppler::usage::InputType::{I16, F32};
use doppler::dsp;

// import external modules
use std::process::exit;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;

extern crate time;
extern crate gpredict;
use gpredict::predict;
use gpredict::tle;

const SPEED_OF_LIGHT_M_S: f64 = 299792458.;
const BUFFER_SIZE: usize = 8192;

macro_rules! println_stderr(
    ($($arg:tt)*) => (
        match writeln!(&mut ::std::io::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
);

fn main() {
    let args = usage::args();

    println_stderr!("doppler {} andres.vahter@gmail.com\n\n", env!("CARGO_PKG_VERSION"));

    let mut stdin = BufReader::with_capacity(BUFFER_SIZE*2, io::stdin());
    let mut stdout = BufWriter::new(io::stdout());
    let mut outbuf: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut samplenr: u32 = 0;

    let mut shift = |intype: doppler::usage::InputType, shift_hz: f64, samplerate: u32| {
        let invec = stdin.by_ref().bytes().take(BUFFER_SIZE).collect::<Result<Vec<u8>,_>>().ok().expect("doppler collect error");

        let freq_shift_fn: fn(&[u8], &mut u32, f64, u32, &mut[u8]) -> (usize, usize) =
            match intype {
                I16 => { dsp::shift_frequency_i16},
                F32 => { dsp::shift_frequency_f32},
        };
        let (sample_count, buflen)  = freq_shift_fn(&invec[..],
                                                   &mut samplenr,
                                                   shift_hz,
                                                   samplerate,
                                                   &mut outbuf);

        stdout.write(&outbuf[0 .. buflen]).map_err(|e|{println_stderr!("doppler stdout.write error: {:?}", e)}).unwrap();
        stdout.flush().map_err(|e|{println_stderr!("doppler stdout.flush error: {:?}", e)}).unwrap();
        (invec.len() != BUFFER_SIZE, sample_count)
    };

    match *args.mode.as_ref().unwrap() {
        ConstMode => {
            println_stderr!("constant shift mode");
            println_stderr!("\tIQ samplerate   : {}", args.samplerate.as_ref().unwrap());
            println_stderr!("\tIQ input type   : {}", args.inputtype.as_ref().unwrap());
            println_stderr!("\tIQ output type  : i16\n");
            println_stderr!("\tfrequency shift : {} Hz", args.constargs.shift.as_ref().unwrap());

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
            println_stderr!("tracking mode");
            println_stderr!("\tIQ samplerate   : {}", args.samplerate.as_ref().unwrap());
            println_stderr!("\tIQ input type   : {}", args.inputtype.as_ref().unwrap());
            println_stderr!("\tIQ output type  : i16\n");
            println_stderr!("\tTLE file        : {}", args.trackargs.tlefile.as_ref().unwrap());
            println_stderr!("\tTLE name        : {}", args.trackargs.tlename.as_ref().unwrap());
            println_stderr!("\tlocation        : {:?}", args.trackargs.location.as_ref().unwrap());
            if args.trackargs.time.is_some() {
                println_stderr!("\ttime            : {:.3}", args.trackargs.time.unwrap().to_utc().rfc3339());
            }
            println_stderr!("\tfrequency       : {} Hz", args.trackargs.frequency.as_ref().unwrap());
            println_stderr!("\toffset          : {} Hz\n\n\n", args.trackargs.offset.unwrap_or(0));

            let l = args.trackargs.location.unwrap();
            let location: predict::Location = predict::Location{lat_deg: l.lat, lon_deg: l.lon, alt_m: l.alt};
            let tlename = args.trackargs.tlename.as_ref().unwrap();
            let tlefile = args.trackargs.tlefile.as_ref().unwrap();

            let tle = match tle::create_tle_from_file(&tlename, &tlefile) {
                Ok(t) => {t},
                Err(e) => {
                    println_stderr!("{}", e);
                    exit(1);
                }
            };

            let mut predict: predict::Predict = predict::Predict::new(tle, location);
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
                            println_stderr!("time                : {:}", (start_time + dt).to_utc().rfc3339());
                            println_stderr!("az                  : {:.2}°", predict.sat.az_deg);
                            println_stderr!("el                  : {:.2}°", predict.sat.el_deg);
                            println_stderr!("range               : {:.0} km", predict.sat.range_km);
                            println_stderr!("range rate          : {:.3} km/sec", predict.sat.range_rate_km_sec);
                            println_stderr!("doppler@{:.3} MHz : {:.2} Hz\n", args.trackargs.frequency.unwrap() as f64 / 1000_000_f64, doppler_hz);
                        }

                        let (stop, count): (bool, usize) = shift(intype, doppler_hz, samplerate);
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
                            println_stderr!("time                : {:}", time::now_utc().to_utc().rfc3339());
                            println_stderr!("az                  : {:.2}°", predict.sat.az_deg);
                            println_stderr!("el                  : {:.2}°", predict.sat.el_deg);
                            println_stderr!("range               : {:.0} km", predict.sat.range_km);
                            println_stderr!("range rate          : {:.3} km/sec", predict.sat.range_rate_km_sec);
                            println_stderr!("doppler@{:.3} MHz : {:.2} Hz\n", args.trackargs.frequency.unwrap() as f64 / 1000_000_f64, doppler_hz);
                        }

                        let (stop, _): (bool, usize) = shift(intype, doppler_hz, samplerate);
                        if stop {
                            break;
                        }
                    }
                }
            };
        }
    }
}
