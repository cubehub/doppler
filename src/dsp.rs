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

use liquid_dsp::LiquidComplex32;

use num::complex::Complex;
use std::mem;
use std::ops::Rem;

// Rust does not support C complex numbers in the same way on 32 and 64 bit platforms:
// https://github.com/rust-lang/rfcs/issues/793
// Therefore this workaround is needed.
//
// Type `LiquidComplex32` is used here because standard `num::complex::Complex` does not have #[repr(C)].
// Function `ccexpf` is implemented in complex.c
//
// Notice that `ccexpf` does not return value, it just changes `z` value in place.
// For some reson it did not work correctly on ARM (BeagleBoneBlack) if `ccexpf` returned
// calculated `LiquidComplex32` struct. Although it worked on Mac OS X and 64/32 bit X86 Ubuntu.
extern {
    pub fn ccexpf(z: *mut LiquidComplex32);
}

use std::f32::consts::PI;

#[cfg(test)]
use std;

#[cfg(test)]
fn assert_eq_delta(a: f32, b: f32, delta: f32) {
    let relative_error = ((a - b) / b).abs();
    if relative_error >= delta {
        panic!("`(left == right)` (left: `{}`, right: `{}`)'", a, b);
    }
}

#[test]
fn test_cexpf() {
    // cargo test -- --nocapture
    // to see prints

    let mut a = Complex::<f32>::new(0.0, 0.0);
    unsafe {ccexpf(mem::transmute(&mut a))};
    assert_eq_delta(a.re, 1.0, 0.000001);
    assert_eq_delta(a.im, 0.0, 0.000001);

    let mut a = Complex::<f32>::new(1.0, 1.0);
    unsafe {ccexpf(mem::transmute(&mut a))};
    assert_eq_delta(a.re, 1.468694, 0.000001);
    assert_eq_delta(a.im, 2.2873552, 0.000001);

    let mut a = Complex::<f32>::new(70.0, 70.0);
    unsafe {ccexpf(mem::transmute(&mut a))};
    assert_eq_delta(a.re, 1593075600000000000000000000000f32, 0.000001);
    assert_eq_delta(a.im, 1946674600000000000000000000000f32, 0.000001);

    let mut a = Complex::<f32>::new(1_000_000.0, 1_000_000.0);
    unsafe {ccexpf(mem::transmute(&mut a))};
    assert_eq!(a.re, std::f32::INFINITY);
    assert_eq!(a.im, -std::f32::INFINITY);

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
        let mut corrector = Complex::<f32>::new(0.0, -2. * PI * (shift_hz / samplerate as f64 * (*samplenum) as f64) as f32);
        unsafe { ccexpf(mem::transmute(&mut corrector))};
        output.push(sample * corrector);

        if (shift_hz / samplerate as f64 * *samplenum as f64).fract() == 0.0 {
            *samplenum = 1;
        }
        else {
            *samplenum += 1;
        }
    }

    output
}

#[test]
fn test_bench_shift_frequency() {
    // use as:
    // cargo test test_bench_shift_frequency -- release

    let mut samplenr: u64 = 0;
    let shift_hz: f64 = 815000.0;
    let samplerate: u32 = 2400000;

    let input: [u8; 1_000_000] = [0xAA; 1_000_000];
    let complex_input = convert_iqf32_to_complex(&input);

    let mut iterator = 0;
    loop {
        shift_frequency(&complex_input, &mut samplenr, shift_hz, samplerate);

        iterator += 1;
        if iterator > 300 {
            break;
        }
    }
}
