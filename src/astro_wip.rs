use astro;
use super::*;
use chrono::{Datelike, Timelike};
use chrono::prelude::*;
use chrono::{DateTime, NaiveDateTime};
use vsop87::vsop87c;

macro_rules! eq_frm_ecl {
    ($ecl_long: expr, $ecl_lat: expr, $oblq_eclip: expr) => {{
        (astro::coords::asc_frm_ecl($ecl_long, $ecl_lat, $oblq_eclip),
         astro::coords::dec_frm_ecl($ecl_long, $ecl_lat, $oblq_eclip))
    }};
}

#[derive(Debug)]
pub struct PlanetInfo{
	pub helio_x: f64,
	pub helio_y: f64,
	pub helio_z: f64,
	pub geo_x: f64,
	pub geo_y: f64,
	pub geo_z: f64,
	pub geo_ecl_long: f64,
	pub geo_ecl_lat: f64,
	pub geo_ecl_dist: f64,
	pub helio_ecl_long: f64,
	pub helio_ecl_lat: f64,
	pub ra: f64,
	pub dec: f64,
	pub geo_lat:f64,
	pub geo_long:f64,
	pub az:f64,
	pub alt:f64,
}

pub fn cart2eq(x:f64, y:f64, z:f64) -> (f64, f64, f64){
    let r = (x * x + y * y + z * z).sqrt();
    let mut ra = (y).atan2(x);
    ra = if ra >= 0.0 {ra} else {(2.0 * PI + ra)};
    let dec = (z / r).asin();
    return (ra, dec, r);
}

pub fn print_planet_info(julian: f64, planet_name: &str, location: Location){
	let info = get_planet_info(julian, planet_name,location);
	let (ra_h,ra_min,ra_sec) = astro::angle::hms_frm_deg(info.ra);
	println!("{:#?}", planet_name);
	println!("{:#?}", info);
	println!("Ra: {:?}:{:?}:{:?}({})",ra_h,ra_min,ra_sec,info.ra);

	// // println!("Ra(geo ecl): {:?}:{:?}:{:?}({})",ra_sun_h,ra_sun_min,ra_sun_sec,info.geo_ecl_long);
	// println!("Dec: {:?}",info.dec);
	// println!("suncalc az: {:?}",suncalc_az.to_degrees()+180.0);
	// println!("suncalc alt: {:?}",suncalc_alt.to_degrees());
	// println!("astro az: {:?}",astro_az.to_degrees()+180.0);
	// println!("Helio Eclp Lat: {:?}",info.helio_ecl_lat);
	// println!("Helio Eclp Long: {:?}",info.helio_ecl_long);
	// println!("");
	// println!("Ra: {:?}:{:?}:{:?},{:?},{:?}",info.geo_ecl_long,ra_h,ra_min,ra_sec, info.geo_ecl_lat, info.geo_ecl_dist );
}

// pub fn print_planet_info_debug(julian: f64, planet_name: &str){
// 	let info = get_planet_info(julian, planet_name);
// 	println!("{:#?}", info);
// 	let (ra_h,ra_min,ra_sec) = astro::angle::hms_frm_deg(info.geo_ecl_long);
// 	println!("({}){:?}:{:?}:{:?},{:?},{:?}",info.geo_ecl_long,ra_h,ra_min,ra_sec, info.geo_ecl_lat, info.geo_ecl_dist );
// }

pub fn get_planet_info(julian: f64, planet_name: &str, location: Location) -> PlanetInfo{
	let lat= location.lat;
	let long= location.lon;

	// let helio_coords = vsop87c::venus(julian);
	let helio_coords = match planet_name {
		"venus" => vsop87c::venus(julian),
		"earth" => vsop87c::earth(julian),
		"mars" => vsop87c::mars(julian),
		"jupiter" => vsop87c::jupiter(julian),
		"mercury" => vsop87c::mercury(julian),
		"saturn" => vsop87c::saturn(julian),
		"uranus" => vsop87c::uranus(julian),
		_ => unreachable!(),
	};
	

	let earth_coords = vsop87c::earth(julian);
    let oblq = astro::ecliptic::mn_oblq_IAU(julian);

	let (geo_x,geo_y,geo_z) = (helio_coords.x-earth_coords.x,helio_coords.y-earth_coords.y,helio_coords.z-earth_coords.z);
	let (geo_ecl_long, geo_ecl_lat, geo_ecl_dist) = cart2eq(geo_x, geo_y, geo_z);
	let (helio_ecl_long, helio_ecl_lat, helio_ecl_dist) = cart2eq(helio_coords.x, helio_coords.y, helio_coords.z);
	// let (geo_ecl_lat, geo_ecl_long, geo_ecl_dist) = cart2eq(helio_coords.x, helio_coords.y, helio_coords.z);
	let (geoeq_asc,geoeq_dec)  = eq_frm_ecl!(geo_ecl_long,geo_ecl_lat,oblq);
	
	// let astro_sidr =  astro::time::mn_sidr(julian);
	// let astro_ha = astro::coords::hr_angl_frm_observer_long(astro_sidr,long,geoeq_asc);
 //    let astro_az =  astro::coords::az_frm_eq(astro_ha,geoeq_dec,lat);

 // 	let (ra_h,ra_min,ra_sec) = astro::angle::hms_frm_deg(info.geo_ecl_long);
	// println!("{}", planet_name);

	let ra_sun = 360.0 + right_ascension(geo_ecl_long,geo_ecl_lat).to_degrees();

	let (ra_sun_h,ra_sun_min,ra_sun_sec) = astro::angle::hms_frm_deg(ra_sun);
	
    let lw  = RAD * -long;
    let phi = RAD * lat;
    let d   = julian_epoch_offset(julian);
	let hour_angle = siderealTime(d, lw) - ra_sun.to_radians();
	let mut suncalc_alt = altitude(hour_angle, phi, geoeq_dec);
    suncalc_alt = suncalc_alt + astroRefraction(suncalc_alt); // altitude correction for refraction
	let suncalc_az = azimuth(hour_angle, phi, geoeq_dec);
    return PlanetInfo{
    	helio_x: helio_coords.x,
    	helio_y: helio_coords.y,
    	helio_z: helio_coords.z,
    	geo_x: geo_x,
    	geo_y: geo_y,
    	geo_z: geo_z,
    	helio_ecl_long: helio_ecl_long.to_degrees(),
    	helio_ecl_lat: helio_ecl_lat.to_degrees(),
    	geo_ecl_long: geo_ecl_long.to_degrees(),
    	geo_ecl_lat: geo_ecl_lat.to_degrees(),
    	geo_ecl_dist: geo_ecl_dist,
    	ra:ra_sun,
    	// ra:geoeq_asc.to_degrees(),
    	dec:geoeq_dec.to_degrees(),
    	geo_lat:lat,
    	geo_long:long,
    	az:suncalc_az.to_degrees()+180.0,
    	alt:suncalc_alt.to_degrees(),

    };
	
}
pub fn moon_pos(julian: f64){
	let lat: f64 = 38.44043;
	let long: f64 = -122.71405;
	// let lw  = (-lng).to_radians();
 	// let phi = (lat).to_radians();
 	let moon_coords = moon::moon_coords(julian);

    // let d   = julian_epoch_offset(julian);
	
    let oblq = astro::ecliptic::mn_oblq_IAU(julian);
	let (geo_ecl, dist) = astro::lunar::geocent_ecl_pos(julian);
	let (geo_eq_asc,geo_eq_dec)  = eq_frm_ecl!(geo_ecl.long,geo_ecl.lat,oblq);
	
	let astro_sidr =  astro::time::mn_sidr(julian);
	let astro_ha = astro::coords::hr_angl_frm_observer_long(astro_sidr,long,geo_eq_asc);
    let astro_az =  astro::coords::az_frm_eq(astro_ha,geo_eq_dec,lat);
    // let hour_angle = siderealTime(d, lw) - geo_eq_asc;
// 
    // let azimuth = azimuth(hour_angle, phi, geo_eq_dec);
	println!("oblq ecleptic{}",oblq);
	println!("suncalc ra, dec, dist \n{},\n{},\n{}", moon_coords.ra, moon_coords.dec, moon_coords.dist);
	println!("suncalc(deg) ra, dec, \n{},\n{}", moon_coords.ra.to_degrees(), moon_coords.dec.to_degrees());
	println!("");
	println!("");
	println!("astro long,lat \n{},\n{},\n{}", geo_ecl.long, geo_ecl.lat, dist);
	println!("");
	println!("astro az {}", astro_az.to_degrees() );
	println!("astro ra, dec, dist \n{},\n{},\n{}", geo_eq_asc, geo_eq_dec, dist);
	println!("astro(deg) ra, dec,  \n{},\n{}", geo_eq_asc.to_degrees(), geo_eq_dec.to_degrees());
	// print!("az rad/deg suncalc{:?}", azimuth.to_degrees());
	// print!("{:?}", azimuth.to_degrees());
}

