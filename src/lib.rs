//! Initially translated from [suncalc.js](http://github.com/mourner/suncalc). Moon phase art by [Joan Stark](https://en.wikipedia.org/wiki/Joan_Stark)

#![allow(non_snake_case)]
extern crate chrono;
use chrono::prelude::*;
use chrono::{DateTime, NaiveDateTime};

mod ascii_art;
use ascii_art::*;
use std::f64::consts::PI;

#[derive(Debug,Clone,Copy)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug,Clone)]
pub struct SolarInfo {
    pub altitude: f64,
    pub azimuth: f64,
}

#[derive(Debug,Clone)]
pub struct LunarInfo {
    pub altitude: f64,
    pub azimuth: f64,
    pub percent_illuminated: f64,
    pub phase_name: String
}

pub fn get_solar_info(julian: f64, location: Location) -> SolarInfo {
    let sun_position = getSunPosition(julian,location.lat,location.lon);
    return SolarInfo {
        altitude:sun_position.altitude.to_degrees(),
        azimuth:sun_position.azimuth.to_degrees() +180.0
    }
}
pub fn get_lunar_info(julian: f64, location: Location) -> LunarInfo {
    let moon_position = getMoonPosition(julian,location.lat,location.lon);
    let phase_name = getMoonPhaseName(getMoonPhase(julian));
    return LunarInfo {
        altitude:moon_position.altitude.to_degrees(),
        azimuth:moon_position.azimuth.to_degrees() +180.0,
        percent_illuminated: getMoonIllumination(julian) * 100.0,
        phase_name: phase_name.to_string(),
    }
}

#[derive(Debug)]
pub struct Coords {
    ra: f64,
    dec: f64,
    dist: f64,
}

#[derive(Debug)]
pub struct Position {
    pub altitude: f64,
    pub azimuth: f64,
}

const MILLLISECONDS_IN_DAY: f64 = 1000.0 * 60.0 * 60.0 * 24.0;
const J1970: f64 = 2440588.0;
const J2000: f64 = 2451545.0;
const RAD: f64 = PI / 180.0;
const OBLIQUITY_OF_EARTH: f64 = RAD * 23.4397;
// const J0: f64 = 0.0009;
// fn julianCycle(d: f64, lw: f64) -> f64 { 
//     return (d - J0 - lw / (2.0 * PI)).round(); 
// }

// fn approxTransit(Ht: f64, lw: f64, n: f64) -> f64 { return J0 + (Ht + lw) / (2.0 * PI) + n; }
// fn solarTransitJ(ds: f64, M: f64, L: f64) -> f64 { 
//     return J2000 + ds + 0.0053 * M.sin() - 0.0069 * (2.0 * L).sin(); 
// }

// fn hourAngle(h: f64, phi: f64, d: f64) -> f64 { 
//     return ((h.sin() - phi.sin() * d.sin()) / (phi.cos() * d.cos())).acos(); 
// }

// // // returns set time for the given sun altitude
// fn getSetJ(h: f64, lw: f64, phi: f64, dec: f64, n: f64, M: f64, L: f64) -> f64 {
//     let w = hourAngle(h, phi, dec);
//     let a = approxTransit(w, lw, n);
//     return solarTransitJ(a, M, L);
// }
pub fn getMoonPhaseImage<'a>(phase_name: &str) -> &'a str {
    let image;
    image = match phase_name {
        "new" => MOON_IMAGE_NEW,
        "waxing crescent" => MOON_IMAGE_WAXING_C,
        "first quarter" => MOON_IMAGE_FIRST_Q,
        "waxing gibbous" => MOON_IMAGE_WAXING_G,
        "full" => MOON_IMAGE_FULL,
        "waning gibbous" => MOON_IMAGE_WANING_G,
        "last quarter" => MOON_IMAGE_LAST_Q,
        "waning crescent" => MOON_IMAGE_WANING_C,
        _ => "no image found",
    };
    return &image;
}

pub fn to_julian(date: DateTime<Local>) -> f64 {
    let ts = (date.timestamp() as f64) * 1000.0;
    return ts / MILLLISECONDS_IN_DAY - 0.5 + J1970;
}
pub fn from_julian(julian: f64) -> NaiveDateTime {
    let millis = ((julian + 0.5 - J1970) * MILLLISECONDS_IN_DAY) as i64 / 1000;
    return NaiveDateTime::from_timestamp(millis, 0);
}

fn julian_epoch_offset(julian: f64) -> f64 {
    return julian - J2000;
}

fn right_ascension(l: f64, b: f64) -> f64 {
    return (l.sin() * OBLIQUITY_OF_EARTH.cos() - b.tan() * OBLIQUITY_OF_EARTH.sin()).atan2(l.cos());
}
fn declination(l: f64, b: f64) -> f64 {
    return (b.sin() * OBLIQUITY_OF_EARTH.cos() + b.cos() * OBLIQUITY_OF_EARTH.sin() * l.sin()).asin();
}

fn solar_mean_anomaly(d: f64) -> f64 {
    return RAD * (357.5291 + 0.98560028 * d);
}

fn ecliptic_longitude(mean_anomaly: f64) -> f64 {
    let C = RAD * (1.9148 * mean_anomaly.sin() + 0.02 * (2.0 * mean_anomaly).sin() + 0.0003 * (3.0 * mean_anomaly).sin());
    let perihelion = RAD * 102.9372; // perihelion of the Earth

    return mean_anomaly + C + perihelion + PI;
}

fn sun_coords(d: f64) -> Coords {

    let M = solar_mean_anomaly(d);
    let L = ecliptic_longitude(M);

    return Coords {
        dec: declination(L, 0.0),
        ra: right_ascension(L, 0.0),
        dist: 149598000.0,
    };
}
fn azimuth(H: f64, phi: f64, dec: f64) -> f64 {
    return H.sin().atan2(H.cos() * phi.sin() - dec.tan() * phi.cos());
}

fn altitude(H: f64, phi: f64, dec: f64) -> f64 {
    return (phi.sin() * dec.sin() + phi.cos() * dec.cos() * H.cos()).asin();
}

fn siderealTime(d: f64, lw: f64) -> f64 {
    return RAD * (280.16 + 360.9856235 * d) - lw;
}

fn astroRefraction(h: f64) -> f64 {
    let alt = if h < 0.0 {0.0} else {h};
    // formula 16.4 of "Astronomical Algorithms" 2nd edition by Jean Meeus (Willmann-Bell, Richmond) 1998.
    // 1.02 / tan(h + 10.26 / (h + 5.10)) h in degrees, result in arc minutes -> converted to RAD:
    return 0.0002967 / (alt + 0.00312536 / (alt + 0.08901179)).tan();
}

pub fn getMoonPosition  (date:f64, lat:f64, lng:f64) -> Position {

    let lw  = RAD * -lng;
    let phi = RAD * lat;
    let d   = julian_epoch_offset(date);

    let c = moon_coords(d);
    let H = siderealTime(d, lw) - c.ra;
    let mut h = altitude(H, phi, c.dec);
    // formula 14.1 of "Astronomical Algorithms" 2nd edition by Jean Meeus (Willmann-Bell, Richmond) 1998.
    // let pa = H.sin().atan2(phi.tan() * c.dec.cos() - c.dec.sin() * H.cos());

    h = h + astroRefraction(h); // altitude correction for refraction
    
    return Position {
        azimuth: azimuth(H, phi, c.dec),
        altitude: h,
    };
    // return {
    //     azimuth: azimuth(H, phi, c.dec),
    //     altitude: h,
    //     distance: c.dist,
    //     parallacticAngle: pa
    // };
}

pub fn getSunPosition  (date:f64, lat:f64, lng:f64) -> Position {

    let lw  = RAD * -lng;
    let   phi = RAD * lat;
    let  d   = julian_epoch_offset(date);

    let  c  = sun_coords(d);
    let  H  = siderealTime(d, lw) - c.ra;

    return Position {
        azimuth: azimuth(H, phi, c.dec),
        altitude: altitude(H, phi, c.dec),
    };
}

pub fn getMoonIllumination(julian: f64) -> f64 {
    let d = julian_epoch_offset(julian);
    let s = sun_coords(d);
    let m = moon_coords(d);

    let phi = (s.dec.sin() * m.dec.sin() + s.dec.cos() * m.dec.cos() * (s.ra - m.ra).cos()).acos();
    let inc = (s.dist * phi.sin()).atan2(m.dist - s.dist * phi.cos());
    let fraction = (1.0 + inc.cos()) / 2.0;

    return fraction;
}

pub fn getMoonPhaseName<'a>(moon_phase: f64) -> &'a str {
    // Floating point ranges not allowed
    let phase = (moon_phase * 100.0) as i32;
    let phase_name = match phase {
        0...03 => "new",
        02...20 => "waxing crescent",
        20...30 => "first quarter",
        30...47 => "waxing gibbous",
        47...53 => "full",
        53...70 => "waning gibbous",
        70...85 => "last quarter",
        85...99 => "waning crescent",
        _ => "anything",
    };
    return phase_name;
}

pub fn getMoonPhase(julian: f64) -> f64 {
    let d = julian_epoch_offset(julian);
    let s = sun_coords(d);
    let m = moon_coords(d);

    let phi = (s.dec.sin() * m.dec.sin() + s.dec.cos() * m.dec.cos() * (s.ra - m.ra).cos()).acos();
    let inc = (s.dist * phi.sin()).atan2(m.dist - s.dist * phi.cos());
    let angle = (s.dec.cos() * (s.ra - m.ra).sin()).atan2(
        (s.dec).sin() * m.dec.cos() -
            s.dec.cos() * (m.dec).sin() *
                (s.ra - m.ra).cos(),
    );

    let some_phase_var = if angle < 0.0 {-1.0} else {1.0};

    return 0.5 + 0.5 * inc * some_phase_var / PI;
}

pub fn moon_coords(d: f64) -> Coords {
    // geocentric ecliptic coordinates of the moon
    let L = RAD * (218.316 + 13.176396 * d); // ecliptic longitude
    let M = RAD * (134.963 + 13.064993 * d); // mean anomaly
    let F = RAD * (93.272 + 13.229350 * d); // mean distance

    let l = L + RAD * 6.289 * M.sin(); // longitude
    let b = RAD * 5.128 * F.sin(); // latitude
    let dt = 385001.0 - 20905.0 * M.cos(); // distance to the moon in km

    return Coords {
        ra: right_ascension(l, b),
        dec: declination(l, b),
        dist: dt,
    };
}
