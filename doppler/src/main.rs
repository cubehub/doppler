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
use doppler::predict;
use doppler::tle;
use doppler::usage;
use doppler::usage::Mode::{ConstMode, TrackMode};

// import external modules
use std::thread;
use std::process::exit;

fn main() {

    let args = usage::args();

    println!("doppler {} andres.vahter@gmail.com\n\n", env!("CARGO_PKG_VERSION"));

    match args.mode.unwrap() {
        ConstMode => {
            println!("constant shift mode");

            println!("\tIQ samplerate   : {}", args.samplerate.as_ref().unwrap());
            println!("\tIQ data type    : {}\n", args.inputtype.as_ref().unwrap());

            println!("\tfrequency shift : {} Hz", args.constargs.shift.as_ref().unwrap());
        },


        TrackMode => {
            println!("tracking mode");

            println!("\tIQ samplerate   : {}", args.samplerate.as_ref().unwrap());
            println!("\tIQ data type    : {}\n", args.inputtype.as_ref().unwrap());

            println!("\tTLE file        : {}", args.trackargs.tlefile.as_ref().unwrap());
            println!("\tTLE name        : {}", args.trackargs.tlename.as_ref().unwrap());
            println!("\tlocation        : {:?}", args.trackargs.location.as_ref().unwrap());
            println!("\ttime            : {:.3}", args.trackargs.time.unwrap_or(0.0));
            println!("\tfrequency       : {} Hz", args.trackargs.frequency.as_ref().unwrap());
            println!("\toffset          : {} Hz\n\n\n", args.trackargs.offset.unwrap_or(0));


            let l = args.trackargs.location.unwrap();
            let location: predict::Location = predict::Location{lat_deg: l.lat, lon_deg: l.lon, alt_m: l.alt};

            let tle = match tle::create_tle_from_file(args.trackargs.tlename.unwrap(), args.trackargs.tlefile.unwrap()) {
                Ok(t) => {t},
                Err(e) => {
                    println!("{}", e);
                    exit(1);
                }
            };

            let mut predict: predict::Predict = predict::Predict::new(tle, location);


            loop {
                predict.update(None);
                println!("az         : {:.2}°", predict.sat.az_deg);
                println!("el         : {:.2}°", predict.sat.el_deg);
                println!("range      : {:.0} km", predict.sat.range_km);
                println!("range rate : {:.3} km/sec\n", predict.sat.range_rate_km_sec);

                thread::sleep_ms(1000);
            }
        },
    }
}
