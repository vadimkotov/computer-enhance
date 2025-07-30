mod haversine;

use rand::Rng;
use std::env;
use std::fs::File;
use std::io::{Write, BufWriter};

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
        let hdist = haversine::reference_haversine(lat1, lon1, lat2, lon2);
        println!("{}", hdist);
        dist_file.write_all(&hdist.to_le_bytes())?;
        // write!(writer, "  {{\"x0\": {}, \"y0\": {}, \"x1\": {}, \"y1\": {}}}", lat1, lon1, lat2, lon2)?; 

        if i < n_samples - 1 {
            writeln!(writer, ",")?;
        } else {
            writeln!(writer)?;
        }
    }

    writeln!(writer, "]}}")?;
    Ok(())
}
