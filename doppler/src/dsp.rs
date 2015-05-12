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


use std::ops::{Add, Sub, Mul};
use std::mem;

// rust currently does not support operations with complex numbers, therefore C functions are used
// C complex wrapper is taken from here, because this lib does not work with beta
// https://github.com/japaric/complex.rs/blob/master/src/lib.rs

/// A complex number in Cartesian form.
#[repr(C)]
#[derive(Clone, Copy, PartialEq)]
pub struct Complex<T> {
    /// The real part
    pub re: T,
    /// The imaginary part
    pub im: T,
}

/// Single precision complex number
#[allow(non_camel_case_types)]
pub type c64 = Complex<f32>;

impl<T> Complex<T> {
    /// Create a new complex number
    pub fn new(re: T, im: T) -> Complex<T> {
        Complex {
            im: im,
            re: re,
        }
    }
}

impl<T> Mul<T> for Complex<T> where T: Clone + Mul<Output=T> {
    type Output = Complex<T>;

    fn mul(self, rhs: T) -> Complex<T> {
        Complex {
            re: self.re * rhs.clone(),
            im: self.im * rhs,
        }
    }
}

impl<T> Mul for Complex<T> where
    T: Add<Output=T> + Clone + Mul<Output=T> + Sub<Output=T>,
{
    type Output = Complex<T>;

    fn mul(self, rhs: Complex<T>) -> Complex<T> {
        Complex {
            re: self.re.clone() * rhs.re.clone() - self.im.clone() * rhs.im.clone(),
            im: self.re * rhs.im + self.im * rhs.re,
        }
    }
}

impl Mul<Complex<f32>> for f32 {
    type Output = Complex<f32>;

    fn mul(self, rhs: Complex<f32>) -> Complex<f32> {
        rhs * self
    }
}

#[link(name="m")]
extern {
    pub fn cexpf(z: c64) -> c64;
}


use std::f32::consts::PI;

pub fn shift_frequency_i16(inbuf: &[u8], samplenr: &mut u32, shift_hz: f64, samplerate: u32, outbuf: &mut[u8]) -> usize {
    // inbuf consists of int16 IQ IQ pairs that are represented as bytes here
    let mut index: usize = 0;

    for b in inbuf.chunks(4) {
        let i: f32 = ((b[1] as i16) << 8 | b[0] as i16) as f32 / 32768.;
        let q: f32 = ((b[3] as i16) << 8 | b[2] as i16) as f32 / 32768.;

        let mut c_sample: c64 = Complex::new(i, q);
        let c_corrector: c64 = unsafe {cexpf(Complex::new(0., -2. * PI * (shift_hz as f32 / samplerate as f32) * *samplenr as f32))};
        c_sample = c_sample * c_corrector;

        let i = (c_sample.re * 32767.0) as i16; // I
        let q = (c_sample.im * 32767.0) as i16; // Q

        outbuf[index + 0] = (i & 0xFF) as u8;
        outbuf[index + 1] = ((i >> 8) & 0xFF) as u8;

        outbuf[index + 2] = (q & 0xFF) as u8;
        outbuf[index + 3] = ((q >> 8) & 0xFF) as u8;

        *samplenr += 1;
        index = index + 4;
    }

    // outbuf len in bytes
    index
}

pub fn shift_frequency_f32(inbuf: &[u8], samplenr: &mut u32, shift_hz: f64, samplerate: u32, outbuf: &mut[u8]) -> usize {
    // inbuf consists of float32 IQ IQ pairs that are represented as bytes here
    let mut index: usize = 0;

    for b in inbuf.chunks(8) {
        let i: f32 = unsafe {mem::transmute::<u32, f32>(((b[3] as u32) << 24) | ((b[2] as u32) << 16) | ((b[1] as u32) << 8) | b[0] as u32)};
        let q: f32 = unsafe {mem::transmute::<u32, f32>(((b[7] as u32) << 24) | ((b[6] as u32) << 16) | ((b[5] as u32) << 8) | b[4] as u32)};

        let mut c_sample: c64 = Complex::new(i, q);
        let c_corrector: c64 = unsafe {cexpf(Complex::new(0., -2. * PI * (shift_hz as f32 / samplerate as f32) * *samplenr as f32))};
        c_sample = c_sample * c_corrector;

        let i = (c_sample.re * 32767.0) as i16; // I
        let q = (c_sample.im * 32767.0) as i16; // Q

        // convert output to i16 format
        outbuf[index + 0] = (i & 0xFF) as u8;
        outbuf[index + 1] = ((i >> 8) & 0xFF) as u8;

        outbuf[index + 2] = (q & 0xFF) as u8;
        outbuf[index + 3] = ((q >> 8) & 0xFF) as u8;

        *samplenr += 1;
        index = index + 4;
    }

    // outbuf len in bytes
    index
}

#[test]
fn test_shift_frequency_i16() {
    let mut inbuf: [u8; 8] = [190, 255, 79, 0, 130, 255, 109, 0];
    let mut outbuf: [u8; 8] = [0; 8];
    let mut samplenr: u32 = 0;

    let len = shift_frequency_i16(&inbuf[0 .. 8], &mut samplenr, 15000 as f64, 126000, &mut outbuf);

    let expectedout: [u8; 8] = [191, 255, 78, 0, 238, 255, 165, 0];

    assert_eq!(outbuf, expectedout);
    assert_eq!(samplenr, 2);
    assert_eq!(len, 8);
}

#[test]
fn test_shift_frequency_f32() {
    let mut inbuf: [u8; 16] = [0, 254, 3, 187, 0, 64, 29, 59, 0, 114, 124, 187, 0, 218, 91, 59];
    let mut outbuf: [u8; 8] = [0; 8];
    let mut samplenr: u32 = 0;

    let len = shift_frequency_f32(&inbuf[0 .. 16], &mut samplenr, 15000 as f64, 126000, &mut outbuf);

    let expectedout: [u8; 8] = [191, 255, 78, 0, 239, 255, 166, 0];

    assert_eq!(outbuf, expectedout);
    assert_eq!(samplenr, 2);
    assert_eq!(len, 8);
}
