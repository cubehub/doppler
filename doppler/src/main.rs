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

extern crate docopt;
mod usage;

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
}
