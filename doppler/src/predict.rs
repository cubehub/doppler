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
use tle;

use libc::{c_double};
use std::default::Default;
use time;

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

fn fraction_of_day(h: i32, m: i32, s: i32) -> f64{
    (h as f64 + (m as f64 + s as f64 / 60.0) / 60.0) / 24.0
}

/// Astronomical Formulae for Calculators, Jean Meeus, pages 23-25.
/// Calculate Julian Date of 0.0 Jan year
fn julian_date_of_year(yr: i32) -> f64 {
    let mut year: u64;
    let mut a: f64;
    let mut b: f64;
    let mut i: f64;

    let mut jdoy: f64;

    year = yr as u64 -1;
    i = (year as f64 / 100.).trunc();
    a = i;
    i = (a / 4.).trunc();
    b = (2. - a + i).trunc();
    i = (365.25 * year as f64).trunc();
    i += (30.6001_f64 * 14.0_f64).trunc();
    jdoy = i + 1720994.5 + b;

    jdoy
}

/// Calculates the day of the year for the specified date.
fn day_of_the_year(yr: i32, mo: i32, dy: i32) -> i32 {
    let days: [u8; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let mut day: i32 = 0;

    for d in &days[0 .. mo as usize - 1] {
        day += *d as i32;
    }

    day += dy as i32;
    if (yr % 4 == 0) && ((yr % 100 != 0) || (yr % 400 == 0)) && (mo > 2) {
        day += 1;
    }

    day
}

// Calculates Julian Day Number
fn julian_day_nr(year: i32, month: i32, day: i32, h: i32, m: i32, s: i32) -> f64 {
    julian_date_of_year(year) + day_of_the_year(year, month, day) as f64 + fraction_of_day(h, m, s)
}

impl Predict {

    pub fn new(tle: tle::Tle, location: Location) -> Predict {
        let tle_t = tle::create_tle_t(tle).unwrap();

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

    pub fn update(&mut self, timeoption: Option<time::Tm>) {
        let juliantime = match timeoption {
            Some(t) => julian_day_nr(t.tm_year+1900, t.tm_mon, t.tm_mday, t.tm_hour, t.tm_min, t.tm_sec),
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

#[test]
fn test_julian_day_nr() {
    // http://en.wikipedia.org/wiki/Julian_day#Converting_Julian_or_Gregorian_calendar_date_to_Julian_Day_Number
    assert_eq!(julian_day_nr(2000, 1, 1, 12, 00, 00), 2451545.0);
    assert_eq!(julian_day_nr(1970, 1, 1, 00, 00, 00), 2440587.5);
}

#[test]
fn test_predict_update() {
    let tle: tle::Tle = tle::Tle{
        name: "ESTCUBE 1".to_string(),
        line1: "1 39161U 13021C   15091.47675532  .00001890  00000-0  31643-3 0  9990".to_string(),
        line2: "2 39161  98.0727 175.0786 0009451 192.0216 168.0788 14.70951130101965".to_string()
    };

    let location: Location = Location{lat_deg:58.64560, lon_deg: 23.15163, alt_m: 8};
    let mut predict: Predict = Predict::new(tle, location);

    predict.update(Some(time::now_utc()));
    println!("az         : {:.*}°", 2, predict.sat.az_deg);
    println!("el         : {:.*}°", 2, predict.sat.el_deg);
    println!("range      : {:.*} km", 0, predict.sat.range_km);
    println!("range rate : {:.*} km/sec\n", 3, predict.sat.range_rate_km_sec);
}
