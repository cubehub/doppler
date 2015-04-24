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

extern crate libc;

#[macro_use]
extern crate clap;

pub mod ffipredict;
pub mod tle;
pub mod predict;
pub mod usage;


#[test]
fn test_predict_update() {
    let tle: tle::Tle = tle::Tle{
        name: "ESTCUBE 1".to_string(),
        line1: "1 39161U 13021C   15091.47675532  .00001890  00000-0  31643-3 0  9990".to_string(),
        line2: "2 39161  98.0727 175.0786 0009451 192.0216 168.0788 14.70951130101965".to_string()
    };

    let location: predict::Location = predict::Location{lat_deg:58.64560, lon_deg: 23.15163, alt_m: 8};
    let mut predict: predict::Predict = predict::Predict::new(tle, location);

    predict.update(Some(0.0));
    println!("az         : {:.*}°", 2, predict.sat.az_deg);
    println!("el         : {:.*}°", 2, predict.sat.el_deg);
    println!("range      : {:.*} km", 0, predict.sat.range_km);
    println!("range rate : {:.*} km/sec\n", 3, predict.sat.range_rate_km_sec);
}
