use vsop87::vsop87c;
use coords::*;

pub fn get_celestial_position(julian: f64, planet_name: &str) -> CelestialPosition {
    let helio_coords = match planet_name {
        "Venus" => vsop87c::venus(julian),
        "Earth" => vsop87c::earth(julian),
        "Mars" => vsop87c::mars(julian),
        "Jupiter" => vsop87c::jupiter(julian),
        "Mercury" => vsop87c::mercury(julian),
        "Saturn" => vsop87c::saturn(julian),
        "Uranus" => vsop87c::uranus(julian),
        "Neptune" => vsop87c::neptune(julian),
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
