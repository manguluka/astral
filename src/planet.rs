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
/// assert_eq!(planet::get_celestial_position(jd,"Mars").julian_day,jd);
/// assert_eq!(planet::get_celestial_position(jd,"mars").julian_day,jd);
/// ```
pub fn get_celestial_position(julian: f64, planet_name: &str) -> CelestialPosition {

    let helio_coords = match planet_name.to_lowercase().as_str() {
        "venus" => vsop87c::venus(julian),
        "earth" => vsop87c::earth(julian),
        "mars" => vsop87c::mars(julian),
        "jupiter" => vsop87c::jupiter(julian),
        "mercury" => vsop87c::mercury(julian),
        "saturn" => vsop87c::saturn(julian),
        "uranus" => vsop87c::uranus(julian),
        "neptune" => vsop87c::neptune(julian),
        _ => unreachable!(),
    };
    let helio_cart_coords = CartesianCoordinates {
        x: helio_coords.x,
        y: helio_coords.y,
        z: helio_coords.z,
    };
    let point =
        CelestialPosition::from_helio_cart(julian, helio_cart_coords, CelestialBodyType::Planet);
    return point;
}
