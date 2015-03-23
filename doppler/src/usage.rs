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

use docopt;
use docopt::Docopt;

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

pub fn args() -> docopt::ArgvMap {
    let args = Docopt::new(USAGE)
                      .and_then(|dopt| dopt.parse())
                      .unwrap_or_else(|e| e.exit());
    args
}
