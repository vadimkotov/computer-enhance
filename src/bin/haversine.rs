const DEG2RAD: f64 = std::f64::consts::PI / 180.0;
const EARTH_RADIUS: f64 = 6372.8;

fn degrees_to_radians(deg: f64) -> f64 {
    deg * DEG2RAD
}

pub fn reference_haversine(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let dlat = degrees_to_radians(lat2 - lat1);
    let dlon = degrees_to_radians(lon2 - lon1);

    let lat1 = degrees_to_radians(lat1);
    let lat2 = degrees_to_radians(lat2);
    
    let a = f64::sin(dlat/2.0).powf(2.0) + f64::cos(lat1) * f64::cos(lat2) * f64::sin(dlon/2.0).powf(2.0);
    let c = 2.0 * f64::asin(f64::sqrt(a));

    c * EARTH_RADIUS
}

