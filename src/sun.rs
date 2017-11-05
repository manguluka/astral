use vsop87::vsop87c;
use util::*;
use coords::*;

pub fn get_celestial_position(julian: f64) -> CelestialPosition {
    let helio_coords = vsop87c::earth(julian);
    let helio_cart_coords = CartesianCoordinates {
        x: -helio_coords.x,
        y: -helio_coords.y,
        z: -helio_coords.z,
    };
    let point = CelestialPosition {
        geo_cart: helio_cart_coords,
        object_type: CelestialBodyType::Sun,
        julian_day: julian,
    };
    return point;
}

fn solar_mean_anomaly(d: f64) -> f64 {
    return RAD * (357.5291 + 0.98560028 * d);
}

pub fn sun_coords(d: f64) -> Coords {

    let M = solar_mean_anomaly(d);
    let L = ecliptic_longitude(M);

    return Coords {
        dec: declination(L, 0.0),
        ra: right_ascension(L, 0.0),
        dist: 149598000.0,
    };
}

pub fn getSunPosition(julian: f64, lat: f64, lng: f64) -> HzCoordinates {
    let lw = RAD * -lng;
    let lat_rad = RAD * lat;
    let d = julian_epoch_offset(julian);

    let sun_coords = sun_coords(d);
    let hour_angle = siderealTime(d, lw) - sun_coords.ra;

    return HzCoordinates {
        az: azimuth(hour_angle, lat_rad, sun_coords.dec),
        alt: altitude(hour_angle, lat_rad, sun_coords.dec),
    };
}
