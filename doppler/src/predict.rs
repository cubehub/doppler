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


use ffipredict;

use std::ffi::{CString};
use libc::{c_char};
use std::default::Default;
use std::slice::bytes::copy_memory;
use std::mem::transmute;

pub struct Tle {
    pub name: String,
    pub line1: String,
    pub line2: String,
}

pub struct Location {
    pub lat_deg: f64,
    pub lon_deg: f64,
    pub alt_m: i32,
}

pub struct Predict {
    pub sat: ffipredict::sat_t,
    qth: ffipredict::qth_t,
}


fn create_tle_t(tle: Tle) -> Result<ffipredict::tle_t, &'static str> {
    let mut tle_t = ffipredict::tle_t {
        epoch: 0.0,
        epoch_year: 0,
        epoch_day: 0,
        epoch_fod: 0.0,
        xndt2o: 0.0,
        xndd6o: 0.0,
        bstar: 0.0,
        xincl: 0.0,
        xnodeo: 0.0,
        eo: 0.0,
        omegao: 0.0,
        xmo: 0.0,
        xno: 0.0,

        catnr: 0,
        elset: 0,
        revnum: 0,

        sat_name: [0; 25],
        idesg: [0; 9],
        status: ffipredict::op_stat_t::OP_STAT_UNKNOWN,

        xincl1: 0.0,
        xnodeo1: 0.0,
        omegao1: 0.0,
        //..Default::default()
    };

    let name = CString::new(tle.name).unwrap();
    let line1 = CString::new(tle.line1).unwrap();
    let line2 = CString::new(tle.line2).unwrap();
    let mut buf = [[0u8; 80]; 3];

    copy_memory(&mut buf[0], name.as_bytes_with_nul());
    copy_memory(&mut buf[1], line1.as_bytes_with_nul());
    copy_memory(&mut buf[2], line2.as_bytes_with_nul());


    let tle_set_result = unsafe { ffipredict::Get_Next_Tle_Set(transmute::<&u8, *const c_char>(&buf[0][0]) , &mut tle_t)};

    if tle_set_result == 1 {
        Ok(tle_t)
    }
    else {
        Err("error in TLE parsing")
    }
}

impl Predict {

    pub fn new(tle: Tle, location: Location) -> Predict {
        let tle_t = create_tle_t(tle).unwrap();

        let sgps: ffipredict::sgpsdp_static_t = Default::default();
        let dps: ffipredict::deep_static_t = Default::default();
        let deep_arg: ffipredict::deep_arg_t = Default::default();
        let pos: ffipredict::vector_t = Default::default();
        let vel: ffipredict::vector_t = Default::default();

        let sat = ffipredict::sat_t{
            name: CString::new("").unwrap().as_ptr(),
            nickname: CString::new("").unwrap().as_ptr(),
            website: CString::new("").unwrap().as_ptr(),
            tle: tle_t,
            flags: 0,
            sgps: sgps,
            dps: dps,
            deep_arg: deep_arg,
            pos: pos,
            vel: vel,

            jul_epoch: 0.0,
            jul_utc: 0.0,
            tsince: 0.0,
            aos: 0.0,
            los: 0.0,
            az: 0.0,
            el: 0.0,
            range: 0.0,
            range_rate: 0.0,
            ra: 0.0,
            dec: 0.0,
            ssplat: 0.0,
            ssplon: 0.0,
            alt: 0.0,
            velo: 0.0,
            ma: 0.0,
            footprint: 0.0,
            phase: 0.0,
            meanmo: 0.0,
            orbit: 0,
            otype: ffipredict::orbit_type_t::ORBIT_TYPE_UNKNOWN,
            //..Default::default()
        };

        let qth = ffipredict::qth_t {
            name: CString::new("").unwrap().as_ptr(),
            loc: CString::new("").unwrap().as_ptr(),
            desc: CString::new("").unwrap().as_ptr(),
            lat: location.lat_deg,
            lon: location.lon_deg,
            alt: location.alt_m,
            qra: CString::new("").unwrap().as_ptr(),
            wx: CString::new("").unwrap().as_ptr(),
        };

        Predict{sat: sat, qth: qth}
    }

    pub fn update(&mut self) {
        //let julian_time_now = unsafe {ffipredict::get_current_daynum()};
        let julian_time_now = 2457112.9;
        println!("julian: {:?}", julian_time_now);
        unsafe {ffipredict::predict_calc(&mut self.sat, &mut self.qth, julian_time_now)};
    }
}
