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
use std::{cmp, ptr};
use std::mem::transmute;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct Tle {
    pub name: String,
    pub line1: String,
    pub line2: String,
}

fn copy_memory(src: &[u8], dst: &mut [u8]) -> usize {
    let len = cmp::min(src.len(), dst.len());
    unsafe {
        ptr::copy_nonoverlapping(&src[0], &mut dst[0], len);
    }
    len
}

fn trim(line: &String) -> String {
    let chars_to_trim: &[char] = &['\r', '\n'];
    let mut l: String;
    l = line.trim_matches(chars_to_trim).to_string();
    l = l.trim_right().to_string();
    l
}

pub fn create_tle_from_file(tlename: &str, pathstr: &str) -> Result<Tle, String> {
    let path = Path::new(&pathstr);
    let file = File::open(&path);
    match file.as_ref() {
        Ok(_) => {}
        Err(_) => {
            return Err(format!("could not open file {}", pathstr))
        }
    }

    let reader = BufReader::new(file.unwrap());
    let mut lines = reader.lines();
    let mut name = String::new();

    for line in &mut lines {
        let mut l = line.unwrap();
        l = trim(&l);

        if l == tlename {
            name = l;
            break;
        }
    }

    if name.len() == 0 {
        Err(format!("{} not found in {}", tlename, pathstr))
    }
    else {
        let tle = Tle { name: name,
                        line1: trim(&lines.next().unwrap().unwrap()),
                        line2: trim(&lines.next().unwrap().unwrap())
                      };

        Ok(tle)
    }
}

pub fn create_tle_t(tle: Tle) -> Result<ffipredict::tle_t, &'static str> {
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

    copy_memory(name.as_bytes_with_nul(), &mut buf[0]);
    copy_memory(line1.as_bytes_with_nul(), &mut buf[1]);
    copy_memory(line2.as_bytes_with_nul(), &mut buf[2]);


    let tle_set_result = unsafe { ffipredict::Get_Next_Tle_Set(transmute::<&u8, *const c_char>(&buf[0][0]), &mut tle_t)};

    if tle_set_result == 1 {
        Ok(tle_t)
    }
    else {
        Err("error in TLE parsing")
    }
}
