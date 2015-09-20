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

use num::complex::Complex;
use std::mem;

// Rust does not support C complex numbers in the same way on 32 and 64 bit platforms:
// https://github.com/rust-lang/rfcs/issues/793
//
// Therefore this workaround is needed:
#[link(name="m")]
#[cfg(target_pointer_width="32")]
extern {
    pub fn cexpf(z: u64) -> u64;
}

#[link(name="m")]
#[cfg(target_pointer_width="64")]
extern {
    pub fn cexpf(z: Complex<f32>) -> Complex<f32>;
}

use std::f32::consts::PI;

#[cfg(test)]
use std;

#[test]
fn test_cexpf() {
    let a: Complex<f32> = unsafe {mem::transmute(cexpf(mem::transmute(Complex::<f32>::new(0.0, 0.0))))};
    assert_eq!(a, Complex::<f32>::new(1.0, 0.0));

    let a: Complex<f32> = unsafe {mem::transmute(cexpf(mem::transmute(Complex::<f32>::new(1.0, 1.0))))};
    assert_eq!(a, Complex::<f32>::new(1.468694, 2.2873552));

    let a: Complex<f32> = unsafe {mem::transmute(cexpf(mem::transmute(Complex::<f32>::new(70.0, 70.0))))};
    assert_eq!(a, Complex::<f32>::new(1593075600000000000000000000000f32, 1946674600000000000000000000000f32));

    let a: Complex<f32> = unsafe {mem::transmute(cexpf(mem::transmute(Complex::<f32>::new(1_000_000.0, 1_000_000.0))))};
    assert_eq!(a, Complex::<f32>::new(std::f32::INFINITY, -std::f32::INFINITY));

    //println!("a={:?}", a);
}

pub fn convert_iqi16_to_complex(inbuf: &[u8]) -> Vec<Complex<f32>> {
    // inbuf consists of i16 IQ pairs that are represented as bytes here
    assert!(inbuf.len() % 4 == 0);

    let mut output = Vec::<Complex<f32>>::with_capacity(inbuf.len()/8);

    for b in inbuf.chunks(4) {
        let i: f32 = ((b[1] as i16) << 8 | b[0] as i16) as f32 / 32768.;
        let q: f32 = ((b[3] as i16) << 8 | b[2] as i16) as f32 / 32768.;

        output.push(Complex::<f32>::new(i, q));
    }

    output
}

pub fn convert_iqf32_to_complex(inbuf: &[u8]) -> Vec<Complex<f32>> {
    // inbuf consists of f32 IQ pairs that are represented as bytes here
    assert!(inbuf.len() % 8 == 0);

    let mut output = Vec::<Complex<f32>>::with_capacity(inbuf.len()/8);

    for b in inbuf.chunks(8) {
        let i: f32 = unsafe {mem::transmute::<u32, f32>(((b[3] as u32) << 24) | ((b[2] as u32) << 16) | ((b[1] as u32) << 8) | b[0] as u32)};
        let q: f32 = unsafe {mem::transmute::<u32, f32>(((b[7] as u32) << 24) | ((b[6] as u32) << 16) | ((b[5] as u32) << 8) | b[4] as u32)};

        output.push(Complex::<f32>::new(i, q));
    }

    output
}

pub fn shift_frequency(inbuf: &[Complex<f32>], samplenum: &mut u64, shift_hz: f64, samplerate: u32) -> Vec<Complex<f32>> {
    let mut output = Vec::<Complex<f32>>::with_capacity(inbuf.len());

    for sample in inbuf {
        let corrector: Complex<f32> = unsafe { mem::transmute(cexpf(mem::transmute(
            Complex::<f32>::new(0., -2. * PI * (shift_hz as f64 / samplerate as f64 * *samplenum as f64) as f32))
        ))};

        output.push(sample * corrector);
        *samplenum += 1;
    }

    // if samplenum grows too big it introduses noise in floating point math therefore samplenum should be zeroed

    // zero after evey period does not work with AX25 decoding, however FM audio is nice
    // there must be some kind of error here:
    // if *samplenum as f32 > samplerate as f32 / shift_hz as f32 {
    //     *samplenum = 0
    // }

    // ... so use this instead:
    if *samplenum > 1_000_000 {
        *samplenum = 0;
    }

    output
}
