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

use libc::{c_void, c_int, c_char, c_double, c_ulong, c_uint};

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum op_stat_t {
    OP_STAT_UNKNOWN = 0,
    OP_STAT_OPERATIONAL,    // Operational           [+]
    OP_STAT_NONOP,          // Nonoperational        [-]
    OP_STAT_PARTIAL,        // Partially operational [P]
    OP_STAT_STDBY,          // Backup/Standby        [B]
    OP_STAT_SPARE,          // Spare                 [S]
    OP_STAT_EXTENDED,       // Extended Mission      [X]
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum orbit_type_t {
    ORBIT_TYPE_UNKNOWN = 0,
    ORBIT_TYPE_LEO,         // Low Earth orbit, up to 1200 km
    ORBIT_TYPE_ICO,         // Intermediate Circular Orbit, up to 1400 km
    ORBIT_TYPE_GEO,         // Geostationary
    ORBIT_TYPE_GSO,         // Geosynchronuous
    ORBIT_TYPE_MOLNIYA,
    ORBIT_TYPE_TUNDRA,
    ORBIT_TYPE_POLAR,
    ORBIT_TYPE_SUNSYNC,
    ORBIT_TYPE_DECAYED
}

#[repr(C)]
#[allow(dead_code)]
pub struct qth_t {
    pub name:       *const c_char,  // Name, eg. callsign
    pub loc:        *const c_char,  // Location, eg City, Country
    pub desc:       *const c_char,  // Short description
    pub lat:        c_double,       // Latitude in dec. deg. North
    pub lon:        c_double,       // Longitude in dec. deg. East
    pub alt:        c_int,          // Altitude above sea level in meters
    pub qra:        *const c_char,  // QRA locator
    pub wx:         *const c_char,  // Weather station code (4 chars)
}

#[repr(C)]
#[allow(dead_code)]
pub struct tle_t {
    pub epoch:      c_double,       // Epoch Time in NORAD TLE format YYDDD.FFFFFFFF
    pub epoch_year: c_uint,         // Epoch: year
    pub epoch_day:  c_uint,         // Epoch: day of year
    pub epoch_fod:  c_double,       // Epoch: Fraction of day
    pub xndt2o:     c_double,       // 1. time derivative of mean motion
    pub xndd6o:     c_double,       // 2. time derivative of mean motion
    pub bstar:      c_double,       // Bstar drag coefficient
    pub xincl:      c_double,       // Inclination
    pub xnodeo:     c_double,       // R.A.A.N.
    pub eo:         c_double,       // Eccentricity
    pub omegao:     c_double,       // argument of perigee
    pub xmo:        c_double,       // mean anomaly
    pub xno:        c_double,       // mean motion

    pub catnr:      c_int,          // Catalogue Number
    pub elset:      c_int,          // Element Set number
    pub revnum:     c_int,          // Revolution Number at epoch

    pub sat_name:   [c_char; 25],   // Satellite name string
    pub idesg:      [c_char; 9],    // International Designator
    pub status:     op_stat_t,      // Operational status

    // values needed for squint calculations
    pub xincl1:     c_double,
    pub xnodeo1:    c_double,
    pub omegao1:    c_double,
}

#[repr(C)]
#[derive(Default)]
#[allow(dead_code)]
pub struct vector_t {
    pub x:          c_double,   // X component
    pub y:          c_double,   // Y component
    pub z:          c_double,   // Z component
    pub w:          c_double,   // Magnitude
}

#[repr(C)]
#[allow(dead_code)]
#[derive(Default)]
/// Common arguments between deep-space functions
pub struct deep_arg_t {
    // Used by dpinit part of Deep()
    pub eosq:       c_double,
    pub sinio:      c_double,
    pub cosio:      c_double,
    pub betao:      c_double,
    pub aodp:       c_double,
    pub theta2:     c_double,
    pub sing:       c_double,
    pub cosg:       c_double,
    pub betao2:     c_double,
    pub xmdot:      c_double,
    pub omgdot:     c_double,
    pub xnodot:     c_double,
    pub xnodp:      c_double,

    // Used by dpsec and dpper parts of Deep()
    pub xll:        c_double,
    pub omgadf:     c_double,
    pub xnode:      c_double,
    pub em:         c_double,
    pub xinc:       c_double,
    pub xn:         c_double,
    pub t:          c_double,

    // Used by thetg and Deep()
    pub ds50:       c_double,
}

#[repr(C)]
#[allow(dead_code)]
#[derive(Default)]
/// static data for SGP4 and SDP4
pub struct sgpsdp_static_t {
    pub aodp:       c_double,
    pub aycof:      c_double,
    pub c1:         c_double,
    pub c4:         c_double,
    pub c5:         c_double,
    pub cosio:      c_double,
    pub d2:         c_double,
    pub d3:         c_double,
    pub d4:         c_double,
    pub delmo:      c_double,
    pub omgcof:     c_double,

    pub eta:        c_double,
    pub omgdot:     c_double,
    pub sinio:      c_double,
    pub xnodp:      c_double,
    pub sinmo:      c_double,
    pub t2cof:      c_double,
    pub t3cof:      c_double,
    pub t4cof:      c_double,
    pub t5cof:      c_double,

    pub x1mth2:     c_double,
    pub x3thm1:     c_double,
    pub x7thm1:     c_double,
    pub xmcof:      c_double,
    pub xmdot:      c_double,
    pub xnodcf:     c_double,
    pub xnodot:     c_double,
    pub xlcof:      c_double,
}

#[repr(C)]
#[allow(dead_code)]
#[derive(Default)]
/// static data for DEEP
pub struct deep_static_t {
    pub thgr:       c_double,
    pub xnq:        c_double,
    pub xqncl:      c_double,
    pub omegaq:     c_double,
    pub zmol:       c_double,
    pub zmos:       c_double,
    pub savtsn:     c_double,
    pub ee2:        c_double,
    pub e3:         c_double,
    pub xi2:        c_double,

    pub xl2:        c_double,
    pub xl3:        c_double,
    pub xl4:        c_double,
    pub xgh2:       c_double,
    pub xgh3:       c_double,
    pub xgh4:       c_double,
    pub xh2:        c_double,
    pub xh3:        c_double,
    pub sse:        c_double,
    pub ssi:        c_double,
    pub ssg:        c_double,
    pub xi3:        c_double,

    pub se2:        c_double,
    pub si2:        c_double,
    pub sl2:        c_double,
    pub sgh2:       c_double,
    pub sh2:        c_double,
    pub se3:        c_double,
    pub si3:        c_double,
    pub sl3:        c_double,
    pub sgh3:       c_double,
    pub sh3:        c_double,
    pub sl4:        c_double,
    pub sgh4:       c_double,

    pub ssl:        c_double,
    pub ssh:        c_double,
    pub d3210:      c_double,
    pub d3222:      c_double,
    pub d4410:      c_double,
    pub d4422:      c_double,
    pub d5220:      c_double,
    pub d5232:      c_double,
    pub d5421:      c_double,

    pub d5433:      c_double,
    pub del1:       c_double,
    pub del2:       c_double,
    pub del3:       c_double,
    pub fasx2:      c_double,
    pub fasx4:      c_double,
    pub fasx6:      c_double,
    pub xlamo:      c_double,
    pub xfact:      c_double,

    pub xni:        c_double,
    pub atime:      c_double,
    pub stepp:      c_double,
    pub stepn:      c_double,
    pub step2:      c_double,
    pub preep:      c_double,
    pub pl:         c_double,
    pub sghs:       c_double,
    pub xli:        c_double,

    pub d2201:      c_double,
    pub d2211:      c_double,
    pub sghl:       c_double,
    pub sh1:        c_double,
    pub pinc:       c_double,
    pub pe:         c_double,
    pub shs:        c_double,
    pub zsingl:     c_double,
    pub zcosgl:     c_double,

    pub zsinhl:     c_double,
    pub zcoshl:     c_double,
    pub zsinil:     c_double,
    pub zcosil:     c_double,

}

#[repr(C)]
pub struct sat_t {
    pub name:       *const c_char,
    pub nickname:   *const c_char,
    pub website:    *const c_char,
    pub tle:        tle_t,          // Keplerian elements
    pub flags:      c_int,          // Flags for algo ctrl
    pub sgps:       sgpsdp_static_t,
    pub dps:        deep_static_t,
    pub deep_arg:   deep_arg_t,
    pub pos:        vector_t,       // Raw position and range
    pub vel:        vector_t,       // Raw velocity

    pub jul_epoch:  c_double,
    pub jul_utc:    c_double,
    pub tsince:     c_double,
    pub aos:        c_double,       // Next AOS
    pub los:        c_double,       // Next LO

    pub az:         c_double,       // Azimuth [deg]
    pub el:         c_double,       // Elevation [deg]
    pub range:      c_double,       // Range [km]
    pub range_rate: c_double,       // Range Rate [km/sec]
    pub ra:         c_double,       // Right Ascension [deg]
    pub dec:        c_double,       // Declination [deg]
    pub ssplat:     c_double,       // SSP latitude [deg]
    pub ssplon:     c_double,       // SSP longitude [deg]
    pub alt:        c_double,       // altitude [km]
    pub velo:       c_double,       // velocity [km/s]
    pub ma:         c_double,       // mean anomaly
    pub footprint:  c_double,       // footprint
    pub phase:      c_double,       // orbit phase
    pub meanmo:     c_double,       // mean motion kept in rev/day
    pub orbit:      c_ulong,        // orbit number
    pub otype:      orbit_type_t    // orbit type
}

#[link(name = "gpredict")]
extern {
    pub fn get_current_daynum() -> c_double;
    pub fn predict_calc(sat: *mut sat_t, qth: *mut qth_t, t: c_double) -> c_void;

    pub fn Get_Next_Tle_Set(line: *const c_char, tle: *mut tle_t) -> c_int;
    pub fn select_ephemeris(sat: *mut sat_t) -> c_void;
    pub fn gtk_sat_data_init_sat(sat: *mut sat_t, sat: *mut qth_t) -> c_void;
}
