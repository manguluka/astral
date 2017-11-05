use ascii_art::*;
use super::*;
use util::*;
use coords::*;
use sun::sun_coords;
use std::f64::consts::PI;

pub fn get_lunar_info(julian: f64, location: Location) -> LunarInfo {
    let moon_position = getMoonPosition(julian, location.lat, location.lon);
    let phase_name = getMoonPhaseName(getMoonPhase(julian));
    let phase_image = getMoonPhaseImage(phase_name);
    return LunarInfo {
        altitude: moon_position.alt,
        azimuth: moon_position.az,
        percent_illuminated: getMoonIllumination(julian) * 100.0,
        phase_name: phase_name.to_string(),
        phase_image: phase_image.to_string(),
    };
}

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

pub fn getMoonPosition(date: f64, lat: f64, lng: f64) -> HzCoordinates {
    let lw = RAD * -lng;
    let phi = RAD * lat;
    let d = julian_epoch_offset(date);
    let c = moon_coords(d);
    let hour_angle = siderealTime(d, lw) - c.ra;
    let mut h = altitude(hour_angle, phi, c.dec);
    h = h + astroRefraction(h); // altitude correction for refraction
    return HzCoordinates {
        az: azimuth(hour_angle, phi, c.dec).to_degrees()+180.0,
        alt: h.to_degrees(),
    };
}

pub fn getMoonIllumination(julian: f64) -> f64 {
    let d = julian_epoch_offset(julian);
    let s = sun::sun_coords(d);
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

    let some_phase_var = if angle < 0.0 { -1.0 } else { 1.0 };
    return 0.5 + 0.5 * inc * some_phase_var / PI;
}

pub fn moon_coords(d: f64) -> Coords {
    // geocentric ecliptic coordinates of the moon
    let L = RAD * (218.316 + 13.176396 * d); // ecliptic longitude
    let M = RAD * (134.963 + 13.064993 * d); // mean anomaly
    let F = RAD * (93.272 + 13.229350 * d); // mean distance

    let l = L + RAD * 6.289 * M.sin(); // longitude
    let b = RAD * 5.128 * F.sin(); // latitude
    // println!("long,lat(deg) {},{}", l.to_degrees(),b.to_degrees());
    let dt = 385001.0 - 20905.0 * M.cos(); // distance to the moon in km

    return Coords {
        ra: right_ascension(l, b),
        dec: declination(l, b),
        dist: dt,
    };
}
