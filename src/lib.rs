//! High level astronomical data library
//! Initially translated from [suncalc.js](http://github.com/mourner/suncalc).
//! Moon phase art by [Joan Stark](https://en.wikipedia.org/wiki/Joan_Stark)
#![feature(use_extern_macros)]
#![allow(non_snake_case)]
#[macro_use]
extern crate assert_approx_eq;
#[macro_use]
extern crate serde_derive;
extern crate astro;
extern crate vsop87;
extern crate chrono;

use chrono::prelude::*;
use chrono::{DateTime, NaiveDateTime};

pub use astro::angle;
pub mod ascii_art;
pub mod moon;
pub mod sun;
pub mod planet;
pub mod star;
#[macro_use]
pub mod util;
pub mod coords;

#[cfg(test)]
mod tests {
    use planet;
    // use sun;
    use star;
    use coords::*;
    // use chrono::prelude::*;
    const JULIAN_DAY: f64 = 2458061.2743171295;
    const LOCATION: Location = Location {
        lat: 38.44043,
        lon: -122.71405,
    };

    #[test]
    fn get_celestial_position_star_test() {
        let celestial = star::get_celestial_position(JULIAN_DAY, "Polaris").unwrap();
        let tolerence = 2.0;
        assert_approx_eq!(37.954522, celestial.get_eq_coords().ra, tolerence);
        assert_approx_eq!(89.264108, celestial.get_eq_coords().dec, tolerence);
    }

    #[test]
    fn get_celestial_position_planet_test() {
        let celestial = planet::get_celestial_position(JULIAN_DAY, "Venus").unwrap();
        let tolerence = 0.004f64;
        let dec = -8.45970181351729;
        let ra = 204.04900114888179;
        let az = 172.90148843919002;
        let alt = 42.86299764225456;
        assert_approx_eq!(dec, celestial.get_eq_coords().dec, tolerence);
        assert_approx_eq!(ra, celestial.get_eq_coords().ra, tolerence);
        assert_approx_eq!(az, celestial.get_hz_coords(LOCATION).az, tolerence);
        assert_approx_eq!(alt, celestial.get_hz_coords(LOCATION).alt, tolerence);
    }
}
