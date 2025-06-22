use rand::Rng;
use std::env;
use std::fs::File;
use std::io::{Write, BufWriter};

const DEG2RAD: f64 = std::f64::consts::PI / 180.0;
const EARTH_RADIUS: f64 = 6372.8;

fn degrees_to_radians(deg: f64) -> f64 {
    deg * DEG2RAD
}

fn reference_haversine(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let dlat = degrees_to_radians(lat2 - lat1);
    let dlon = degrees_to_radians(lon2 - lon1);

    let lat1 = degrees_to_radians(lat1);
    let lat2 = degrees_to_radians(lat2);
    
    let a = f64::sin(dlat/2.0).powf(2.0) + f64::cos(lat1) * f64::cos(lat2) * f64::sin(dlon/2.0).powf(2.0);
    let c = 2.0 * f64::asin(f64::sqrt(a));

    c * EARTH_RADIUS
}

fn main() -> Result<(), Box<dyn std::error::Error>> { 

    let n_samples: usize = match env::args().nth(1) {
        Some(n_str) => n_str.parse()?,
        None => { 
            println!("Usage: haversine-generator N_SAMPLES");
            return Ok(());
        },
    };

    let name = format!("haversine_{}", n_samples);
    let json_name = format!("{}.json", name);
    let dist_name = format!("{}.f64", name);
    let mut writer = BufWriter::new(File::create(json_name)?);
    let mut dist_file = File::create(dist_name)?;
    
    let mut rng = rand::rng();
    writeln!(writer, "{{\"pairs\": [")?;
    for i in 0..n_samples {

        let (lat1, lon1, lat2, lon2) = (
            rng.random_range(-90.0..=90.0), 
            rng.random_range(-180.0..=180.0),
            rng.random_range(-90.0..=90.0), 
            rng.random_range(-180.0..=180.0),
        );
        let hdist = reference_haversine(lat1, lon1, lat2, lon2);
        println!("{}", hdist);
        dist_file.write_all(&hdist.to_le_bytes())?;
        write!(writer, "  {{\"x0\": {}, \"y0\": {}, \"x1\": {}, \"y1\": {}}}", lat1, lon1, lat2, lon2)?; 

        if i < n_samples - 1 {
            writeln!(writer, ",")?;
        } else {
            writeln!(writer)?;
        }
    }

    writeln!(writer, "]}}")?;
    Ok(())
}
