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
use doppler::usage as usage;

// import external modules
use std::old_io::Timer;
use std::time::Duration;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");


fn main() {
    let args = usage::args();

    //println!("{:?}", args);

    println!("doppler {} andres.vahter@gmail.com\n\n", VERSION);

    if args.get_bool("const") {
        println!("constant shift mode");

        println!("\tIQ samplerate   : {}", args.get_str("--samplerate"));
        println!("\tIQ data type    : {}\n", args.get_str("--intype"));

        println!("\tfrequency shift : {} Hz", args.get_str("--shift"));
    }
    else if args.get_bool("track") {
        println!("tracking mode");

        println!("\tIQ samplerate   : {}", args.get_str("--samplerate"));
        println!("\tIQ data type    : {}\n", args.get_str("--intype"));

        println!("\tTLE file        : {}", args.get_str("--tlefile"));
        println!("\tTLE name        : {}", args.get_str("--tlename"));
        println!("\tlocation        : {}", args.get_str("--location"));
        println!("\ttime            : {}", args.get_str("--time"));
        println!("\tfrequency       : {} Hz", args.get_str("--freq"));
        println!("\tfrequency shift : {} Hz", args.get_str("--shift"));
    }

    let tle: predict::Tle = predict::Tle{
        name: "ESTCUBE 1".to_string(),
        line1: "1 39161U 13021C   15048.48339150  .00001629  00000-0  27460-3 0  9998".to_string(),
        line2: "2 39161  98.0776 132.4584 0009543 342.5605  17.5261 14.70812859 95643".to_string()
    };

    let location: predict::Location = predict::Location{lat_deg:58.64560, lon_deg: 23.15163, alt_m: 8};
    let mut predict: predict::Predict = predict::Predict::new(tle, location);

    let mut timer = Timer::new().unwrap();
    let periodic = timer.periodic(Duration::milliseconds(1000));

    loop {
        periodic.recv().unwrap();
        predict.update();
        println!("{:?}", predict.sat.alt);
    }
}
