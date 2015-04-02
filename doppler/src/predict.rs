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
use libc::{c_char, c_double};
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

#[derive(Default)]
pub struct Sat {
    /// next AOS
    pub aos:                f64,

    /// next LOS
    pub los:                f64,

    /// azimuth [deg]
    pub az_deg:             f64,

    /// elevation [deg]
    pub el_deg:             f64,

    /// range [km]
    pub range_km:           f64,

    /// range rate [km/sec]
    pub range_rate_km_sec:  f64,

    /// SSP latitude [deg]
    pub lat_deg:            f64,

    /// SSP longitude [deg]
    pub lon_deg:            f64,

    /// altitude [km]
    pub alt_km:             f64,

    /// velocity [km/s]
    pub vel_km_s:           f64,

    /// orbit number
    pub orbit_nr:           u64,
}

pub struct Predict {
    pub sat: Sat,

    p_sat: ffipredict::sat_t,
    p_qth: ffipredict::qth_t,
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


    let tle_set_result = unsafe { ffipredict::Get_Next_Tle_Set(transmute::<&u8, *const c_char>(&buf[0][0]), &mut tle_t)};

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

        let mut sat_t = ffipredict::sat_t{
            name: b"placeholder\0".as_ptr() as *const i8,
            nickname: b"placeholder\0".as_ptr() as *const i8,
            website: b"placeholder\0".as_ptr() as *const i8,
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
        };

        let sat: Sat = Default::default();
        let mut qth = ffipredict::qth_t {
            name: b"placeholder\0".as_ptr() as *const i8,
            loc: b"placeholder\0".as_ptr() as *const i8,
            desc: b"placeholder\0".as_ptr() as *const i8,
            lat: location.lat_deg,
            lon: location.lon_deg,
            alt: location.alt_m,
            qra: b"placeholder\0".as_ptr() as *const i8,
            wx: b"placeholder\0".as_ptr() as *const i8,
        };

        unsafe {ffipredict::select_ephemeris(&mut sat_t)};
        unsafe {ffipredict::gtk_sat_data_init_sat(&mut sat_t, &mut qth)};

        Predict{sat: sat, p_sat: sat_t, p_qth: qth}
    }

    pub fn update(&mut self, timeoption: Option<c_double>) {
        let juliantime  = match timeoption {
            Some(t) => t,
            None => unsafe {ffipredict::get_current_daynum()}
        };

        unsafe {ffipredict::predict_calc(&mut self.p_sat, &mut self.p_qth, juliantime)};

        self.sat.aos                = self.p_sat.aos;
        self.sat.los                = self.p_sat.los;
        self.sat.az_deg             = self.p_sat.az;
        self.sat.el_deg             = self.p_sat.el;
        self.sat.range_km           = self.p_sat.range;
        self.sat.range_rate_km_sec  = self.p_sat.range_rate;
        self.sat.lat_deg            = self.p_sat.ssplat;
        self.sat.lon_deg            = self.p_sat.ssplon;
        self.sat.alt_km             = self.p_sat.alt;
        self.sat.vel_km_s           = self.p_sat.velo;
        self.sat.orbit_nr           = self.p_sat.orbit;
    }
}
