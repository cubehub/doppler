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
use doppler::predict as predict;
use doppler::tle as tle;
use doppler::usage as usage;
use doppler::usage::Mode::{ConstMode, TrackMode};

// import external modules
use std::thread;

fn main() {

    let args = usage::args();

    println!("doppler {} andres.vahter@gmail.com\n\n", env!("CARGO_PKG_VERSION"));

    match args.mode.unwrap() {
        ConstMode => {
            println!("constant shift mode");

            println!("\tIQ samplerate   : {}", args.samplerate.unwrap());
            println!("\tIQ data type    : {}\n", args.inputtype.unwrap());

            println!("\tfrequency shift : {} Hz", args.constargs.shift.unwrap());
        },

        TrackMode => {
            println!("tracking mode");

            //println!("\tIQ samplerate   : {}", args.samplerate);
            //println!("\tIQ data type    : {}\n", args.inputtype);

            /*println!("\tTLE file        : {}", args.get_str("--tlefile"));
            println!("\tTLE name        : {}", args.get_str("--tlename"));
            println!("\tlocation        : {}", args.get_str("--location"));
            println!("\ttime            : {}", args.get_str("--time"));
            println!("\tfrequency       : {} Hz", args.get_str("--freq"));
            println!("\tfrequency shift : {} Hz\n\n\n", args.get_str("--shift"));*/
        },
    }

    /*let location: predict::Location = predict::Location{lat_deg:58.64560, lon_deg: 23.15163, alt_m: 8};
    let tle = tle::create_tle_from_file(args.get_str("--tlename").to_string(), args.get_str("--tlefile").to_string()).unwrap();
    let mut predict: predict::Predict = predict::Predict::new(tle, location);

    loop {
        predict.update(None);
        println!("az         : {:.2}°", predict.sat.az_deg);
        println!("el         : {:.2}°", predict.sat.el_deg);
        println!("range      : {:.0} km", predict.sat.range_km);
        println!("range rate : {:.3} km/sec\n", predict.sat.range_rate_km_sec);

        thread::sleep_ms(1000);
    }
    */

}
