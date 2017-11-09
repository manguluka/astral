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

/// Returns the CelestialPosition of a star
///
/// # Arguments
///
/// * `julian` - Julian day
/// * `planet_name` - Star Name
///
/// # Example
///
/// ```
/// use astral::star;
/// let jd = 24000.0;
/// assert_eq!(star::get_celestial_position(jd,"Polaris").unwrap().julian_day,jd);
/// assert_eq!(star::get_celestial_position(jd,"polaris").unwrap().julian_day,jd);
/// ```
pub fn get_celestial_position(julian: f64, name: &str) -> Result<CelestialPosition, &'static str> {
    match star::get_data(name.to_string()) {
        Ok(star_eq) => {
            let position =  CelestialPosition {
                geo_cart: CartesianCoordinates {
                    x: star_eq.x,
                    y: star_eq.y,
                    z: star_eq.z,
                },
                object_type: CelestialBodyType::Star,
                julian_day: julian,
            };
            Ok(position)
        },
        Err(err) => Err(err),
    }
}

pub fn get_data(name: String) -> Result<StarData, &'static str> {
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
    let found_star = stars.into_iter().find(|ref mut item| {
        item.proper.to_lowercase() == name.to_lowercase()
    });
    if let Some(star) = found_star {
        Ok(star)
    } else{
        Err("No star with that name found")
    }
    // return star.unwrap();
}
