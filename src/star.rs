extern crate csv;
use super::*;
use coords::*;

#[derive(Debug, Clone, Deserialize)]
pub struct StarData {
    proper: String,
    x: f64,
    y: f64,
    z: f64,
    dist: f64,
    ra: f64,
    dec: f64,
}

pub fn get_celestial_position(julian: f64, name: &str) -> CelestialPosition {
    let star_eq = star::get_data(name.to_string());
    return CelestialPosition {
        geo_cart: CartesianCoordinates {
            x: star_eq.x,
            y: star_eq.y,
            z: star_eq.z,
        },
        object_type: CelestialBodyType::Star,
        julian_day: julian,
    };
}

pub fn get_data(name: String) -> StarData {
    let mut stars: Vec<StarData> = vec![];
    let star_csv_string = include_str!("../data/star_data.csv");
    let mut rdr = csv::ReaderBuilder::new().has_headers(true).from_reader(
        star_csv_string
            .as_bytes(),
    );
    for result in rdr.deserialize() {
        let record: StarData = result.unwrap();
        stars.push(record.clone());
    }
    let star = stars.into_iter().find(|ref mut item| item.proper == name);
    return star.unwrap();
}
