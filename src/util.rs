use super::*;
use coords::*;
use std::f64::consts::PI;
#[macro_export]
macro_rules! eq_frm_ecl {
    ($ecl_long: expr, $ecl_lat: expr, $oblq_eclip: expr) => {{
        (astro::coords::asc_frm_ecl($ecl_long, $ecl_lat, $oblq_eclip),
         astro::coords::dec_frm_ecl($ecl_long, $ecl_lat, $oblq_eclip))
    }};
}

pub const MILLLISECONDS_IN_DAY: f64 = 1000.0 * 60.0 * 60.0 * 24.0;
pub const J1970: f64 = 2440588.0;
pub const J2000: f64 = 2451545.0;
pub const RAD: f64 = PI / 180.0;
pub const OBLIQUITY_OF_EARTH: f64 = RAD * 23.4397;
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

pub fn geo_frm_helio(julian_day: f64, helio_coords: CartesianCoordinates) -> CartesianCoordinates {
    let earth_coords = vsop87::vsop87c::earth(julian_day);
    return CartesianCoordinates {
        x: helio_coords.x - earth_coords.x,
        y: helio_coords.y - earth_coords.y,
        z: helio_coords.z - earth_coords.z,
    };
}

pub fn cart_frm_eq(eq_coords: EqCoordinates) -> CartesianCoordinates {
    let x = eq_coords.dist * eq_coords.dec.cos() * eq_coords.ra.cos();
    let y = eq_coords.dist * eq_coords.dec.cos() * eq_coords.ra.sin();
    let z = eq_coords.dist * eq_coords.dec.sin();
    return CartesianCoordinates { x: x, y: y, z: z };
}
pub fn helio_frm_geo(julian_day: f64, geo_coords: CartesianCoordinates) -> CartesianCoordinates {
    let earth_coords = vsop87::vsop87c::earth(julian_day);
    return CartesianCoordinates {
        x: geo_coords.x + earth_coords.x,
        y: geo_coords.y + earth_coords.y,
        z: geo_coords.z + earth_coords.z,
    };
}
pub fn ecl_frm_cart(x: f64, y: f64, z: f64) -> EclCoordinates {
    let r = (x * x + y * y + z * z).sqrt();
    let mut ra = (y).atan2(x);
    ra = if ra >= 0.0 { ra } else { (2.0 * PI + ra) };
    let dec = (z / r).asin();
    return EclCoordinates {
        lat: dec,
        lng: ra,
        dist: r,
    };
}
pub fn eq_frm_cart(cart_coords: CartesianCoordinates) -> EqCoordinates {
    let (x, y, z) = (cart_coords.x, cart_coords.y, cart_coords.z);
    let r = (x * x + y * y + z * z).sqrt();
    let mut ra = (y).atan2(x);
    ra = if ra >= 0.0 { ra } else { (2.0 * PI + ra) };
    let dec = (z / r).asin();
    // let ra=(r).atan2(z);
    // let dec=(y).atan2(x);
    return EqCoordinates {
        dec: dec.to_degrees(),
        ra: ra.to_degrees(),
        dist: r,
    };
}

pub fn to_julian(date: DateTime<FixedOffset>) -> f64 {
    let ts = (date.timestamp() as f64) * 1000.0;
    return ts / MILLLISECONDS_IN_DAY - 0.5 + J1970;
}

pub fn from_julian(julian: f64) -> NaiveDateTime {
    let millis = ((julian + 0.5 - J1970) * MILLLISECONDS_IN_DAY) as i64 / 1000;
    return NaiveDateTime::from_timestamp(millis, 0);
}

pub fn julian_epoch_offset(julian: f64) -> f64 {
    return julian - J2000;
}

pub fn right_ascension(l: f64, b: f64) -> f64 {
    return (l.sin() * OBLIQUITY_OF_EARTH.cos() - b.tan() * OBLIQUITY_OF_EARTH.sin()).atan2(l.cos());
}

pub fn declination(l: f64, b: f64) -> f64 {
    return (b.sin() * OBLIQUITY_OF_EARTH.cos() + b.cos() * OBLIQUITY_OF_EARTH.sin() * l.sin())
        .asin();
}

pub fn ecliptic_longitude(mean_anomaly: f64) -> f64 {
    let C = RAD *
        (1.9148 * mean_anomaly.sin() + 0.02 * (2.0 * mean_anomaly).sin() +
             0.0003 * (3.0 * mean_anomaly).sin());
    let perihelion = RAD * 102.9372; // perihelion of the Earth

    return mean_anomaly + C + perihelion + PI;
}

pub fn azimuth(H: f64, phi: f64, dec: f64) -> f64 {
    return H.sin().atan2(H.cos() * phi.sin() - dec.tan() * phi.cos());
}

pub fn altitude(H: f64, phi: f64, dec: f64) -> f64 {
    return (phi.sin() * dec.sin() + phi.cos() * dec.cos() * H.cos()).asin();
}

pub fn siderealTime(d: f64, lw: f64) -> f64 {
    return RAD * (280.16 + 360.9856235 * d) - lw;
}

pub fn astroRefraction(h: f64) -> f64 {
    let alt = if h < 0.0 { 0.0 } else { h };
    return 0.0002967 / (alt + 0.00312536 / (alt + 0.08901179)).tan();
}
