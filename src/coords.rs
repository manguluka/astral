use super::*;
#[derive(Serialize,Deserialize,Debug, Clone, Copy, PartialEq)]
pub struct Location {
    pub lat: f64,
    pub lon: f64,
}
#[derive(Debug, Clone)]
pub struct LunarInfo {
    pub altitude: f64,
    pub azimuth: f64,
    pub percent_illuminated: f64,
    pub phase_name: String,
    pub phase_image: String,
}

#[derive(Debug)]
pub struct Coords {
    pub ra: f64,
    pub dec: f64,
    pub dist: f64,
}

#[derive(Serialize,Deserialize,Debug, Clone)]
pub struct CartesianCoordinates {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
#[derive(Serialize,Deserialize,Debug, Clone)]
pub struct HzCoordinates {
    pub az: f64,
    pub alt: f64,
}
#[derive(Serialize,Deserialize,Debug, Clone)]
pub struct EclCoordinates {
    pub lat: f64,
    pub lng: f64,
    pub dist: f64,
}
#[derive(Serialize,Deserialize,Debug, Clone)]
pub struct EqCoordinates {
    pub ra: f64,
    pub dec: f64,
    pub dist: f64,
}

#[derive(Debug, Clone)]
pub enum CelestialBodyType {
    Planet,
    Star,
    Sun,
    Moon,
}

#[derive(Debug, Clone)]
pub struct CelestialPosition {
    pub geo_cart: CartesianCoordinates,
    pub object_type: CelestialBodyType,
    pub julian_day: f64,
}

impl CelestialPosition {
    pub fn from_helio_cart(
        julian_day: f64,
        helio_cart_coords: CartesianCoordinates,
        object_type: CelestialBodyType,
    ) -> Self {
        let geo_coords = util::geo_frm_helio(julian_day, helio_cart_coords);
        return CelestialPosition {
            geo_cart: geo_coords,
            object_type: object_type,
            julian_day: julian_day,
        };
    }
    pub fn from_geo_eq(
        julian_day: f64,
        eq_coords: EqCoordinates,
        object_type: CelestialBodyType,
    ) -> Self {
        let geo_coords = util::cart_frm_eq(eq_coords);
        return CelestialPosition {
            geo_cart: geo_coords,
            object_type: object_type,
            julian_day: julian_day,
        };
    }
    pub fn get_hz_coords(&self, location: Location) -> HzCoordinates {
        let eq_coords = self.get_eq_coords();
        let lw = util::RAD * -location.lon;
        let phi = util::RAD * location.lat;
        let d = util::julian_epoch_offset(self.julian_day);
        let hour_angle = util::siderealTime(d, lw) - eq_coords.ra.to_radians();
        let mut alt = util::altitude(hour_angle, phi, eq_coords.dec.to_radians());
        alt = alt + util::astroRefraction(alt); // altitude correction for refraction
        let az = util::azimuth(hour_angle, phi, eq_coords.dec.to_radians());
        return HzCoordinates {
            az: az.to_degrees() + 180.0,
            alt: alt.to_degrees(),
        };
    }
    pub fn get_ecl_coords(&self) -> EclCoordinates {
        let mut coords = util::ecl_frm_cart(self.geo_cart.x, self.geo_cart.y, self.geo_cart.z);
        coords.lat = coords.lat.to_degrees();
        coords.lng = coords.lng.to_degrees();
        return coords;
    }
    pub fn get_eq_coords(&self) -> EqCoordinates {
        let coords = match self.object_type {
            CelestialBodyType::Star => util::eq_frm_cart(self.geo_cart.clone()),
            CelestialBodyType::Planet |
            CelestialBodyType::Sun => {
                let ecl_coords = self.get_ecl_coords();
                let oblq = astro::ecliptic::mn_oblq_IAU(self.julian_day);
                let (_, geoeq_dec) = eq_frm_ecl!(
                    ecl_coords.lng.to_radians(),
                    ecl_coords.lat.to_radians(),
                    oblq
                );
                let ra_sun = 360.0 +
                    util::right_ascension(ecl_coords.lng.to_radians(), ecl_coords.lat.to_radians())
                        .to_degrees();
                return EqCoordinates {
                    ra: ra_sun,
                    dec: geoeq_dec.to_degrees(),
                    dist: ecl_coords.dist,
                };
            }
            _ => unreachable!(),
        };
        return coords;
    }
}


