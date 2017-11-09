use vsop87::vsop87c;
use coords::*;

/// Returns the CelestialPosition of a planet
///
/// # Arguments
///
/// * `julian` - Julian day
/// * `planet_name` - Planet Name
///
/// # Example
///
/// ```
/// use astral::planet;
/// let jd = 24000.0;
/// assert_eq!(planet::get_celestial_position(jd,"Mars").unwrap().julian_day,jd);
/// assert_eq!(planet::get_celestial_position(jd,"mars").unwrap().julian_day,jd);
/// ```
pub fn get_celestial_position(julian: f64, planet_name: &str) -> Result<CelestialPosition, &'static str>  {

    if let Some(helio_coords) = match planet_name.to_lowercase().as_str() {
        "venus" => Some(vsop87c::venus(julian)),
        "earth" => Some(vsop87c::earth(julian)),
        "mars" => Some(vsop87c::mars(julian)),
        "jupiter" => Some(vsop87c::jupiter(julian)),
        "mercury" => Some(vsop87c::mercury(julian)),
        "saturn" => Some(vsop87c::saturn(julian)),
        "uranus" => Some(vsop87c::uranus(julian)),
        "neptune" => Some(vsop87c::neptune(julian)),
        _ => None,
    } {
        println!("test");

        let helio_cart_coords = CartesianCoordinates {
            x: helio_coords.x,
            y: helio_coords.y,
            z: helio_coords.z,
        };
        let point =
            CelestialPosition::from_helio_cart(julian, helio_cart_coords, CelestialBodyType::Planet);
        Ok(point)
    } else {
        Err("Couldnt find planet.")
    }
}
