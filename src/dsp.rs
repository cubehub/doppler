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

#[link(name="m")]
extern {
    pub fn cexpf(z: Complex<f32>) -> Complex<f32>;
}

use std::f32::consts::PI;

/// return:
/// (sample count, number of bites in outbuf)
pub fn shift_frequency_i16(inbuf: &[u8], shift_hz: f64, samplerate: u32, outbuf: &mut[u8]) -> (usize, usize) {
    // inbuf consists of int16 IQ IQ pairs that are represented as bytes here
    let mut index: usize = 0;
    let mut samplenr = 0u32;

    for b in inbuf.chunks(4) {
        let i: f32 = ((b[1] as i16) << 8 | b[0] as i16) as f32 / 32768.;
        let q: f32 = ((b[3] as i16) << 8 | b[2] as i16) as f32 / 32768.;

        let mut c_sample: Complex<f32> = Complex::new(i, q);
        let c_corrector: Complex<f32> = unsafe {cexpf(Complex::new(0., -2. * PI * (shift_hz as f32 / samplerate as f32) * samplenr as f32))};
        c_sample = c_sample * c_corrector;

        let i = (c_sample.re * 32767.0) as i16; // I
        let q = (c_sample.im * 32767.0) as i16; // Q

        outbuf[index + 0] = (i & 0xFF) as u8;
        outbuf[index + 1] = ((i >> 8) & 0xFF) as u8;

        outbuf[index + 2] = (q & 0xFF) as u8;
        outbuf[index + 3] = ((q >> 8) & 0xFF) as u8;

        samplenr += 1;
        index = index + 4;
    }

    // (sample count, number of bytes in outbuf)
    (index/4, index)
}

/// return:
/// (sample count, number of bites in outbuf)
pub fn shift_frequency_f32(inbuf: &[u8], shift_hz: f64, samplerate: u32, outbuf: &mut[u8]) -> (usize, usize) {
    // inbuf consists of float32 IQ IQ pairs that are represented as bytes here
    let mut index: usize = 0;
    let mut samplenr = 0u32;

    for b in inbuf.chunks(8) {
        let i: f32 = unsafe {mem::transmute::<u32, f32>(((b[3] as u32) << 24) | ((b[2] as u32) << 16) | ((b[1] as u32) << 8) | b[0] as u32)};
        let q: f32 = unsafe {mem::transmute::<u32, f32>(((b[7] as u32) << 24) | ((b[6] as u32) << 16) | ((b[5] as u32) << 8) | b[4] as u32)};

        let mut c_sample = Complex::<f32>::new(i, q);
        let c_corrector = unsafe {cexpf(Complex::<f32>::new(0., -2. * PI * (shift_hz as f32 / samplerate as f32) * samplenr as f32))};
        c_sample = c_sample * c_corrector;

        let i = (c_sample.re * 32767.0) as i16; // I
        let q = (c_sample.im * 32767.0) as i16; // Q

        // convert output to i16 format
        outbuf[index + 0] = (i & 0xFF) as u8;
        outbuf[index + 1] = ((i >> 8) & 0xFF) as u8;

        outbuf[index + 2] = (q & 0xFF) as u8;
        outbuf[index + 3] = ((q >> 8) & 0xFF) as u8;

        samplenr += 1;
        index = index + 4;
    }

    // (sample count, number of bytes in outbuf)
    (index/4, index)
}

#[test]
fn test_shift_frequency_i16() {
    let inbuf: [u8; 8] = [190, 255, 79, 0, 130, 255, 109, 0];
    let mut outbuf: [u8; 8] = [0; 8];

    let (sample_count, buflen) = shift_frequency_i16(&inbuf[0 .. 8], 15000 as f64, 126000, &mut outbuf);

    let expectedout: [u8; 8] = [191, 255, 78, 0, 238, 255, 165, 0];

    assert_eq!(outbuf, expectedout);
    assert_eq!(buflen, 8);
    assert_eq!(sample_count, 2);
}

#[test]
fn test_shift_frequency_f32() {
    let inbuf: [u8; 16] = [0, 254, 3, 187, 0, 64, 29, 59, 0, 114, 124, 187, 0, 218, 91, 59];
    let mut outbuf: [u8; 8] = [0; 8];

    let (sample_count, buflen) = shift_frequency_f32(&inbuf[0 .. 16], 15000 as f64, 126000, &mut outbuf);

    let expectedout: [u8; 8] = [191, 255, 78, 0, 239, 255, 166, 0];

    assert_eq!(outbuf, expectedout);
    assert_eq!(buflen, 8);
    assert_eq!(sample_count, 2);
}
