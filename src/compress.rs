use std::{
    collections::HashMap,
    fs,
    sync::{Arc, Mutex},
    thread,
};

use geo::Simplify;

use crate::{countries::CountryCode, shapes::SerializableMultiPolygon};

pub fn compress_and_save(desired_size: usize, countries: HashMap<CountryCode, geo::MultiPolygon>) {
    let optimal_compression = Arc::new(Mutex::new(HashMap::new()));

    let mut handles = Vec::new();
    countries
        .iter()
        .map(|(code, polygon)| (code.clone(), polygon.clone()))
        .collect::<Vec<_>>()
        .chunks(32)
        .for_each(|ch| {
            let optimal_compression = optimal_compression.clone();
            let ch = ch.to_vec();
            handles.push(thread::spawn(move || {
                for country in ch {
                    let (mut optimal, mut delta, mut polygon) = (0.0, i64::MAX, None);

                    for epsilon in 0..=1000 {
                        let country = country.1.clone().simplify(&(epsilon as f64 / 1000.0));
                        let str = serde_json::to_string(&SerializableMultiPolygon(country.clone()))
                            .unwrap();
                        let new_size = str.len() as i64;

                        // make the size as close to 60kb as possible
                        if (desired_size as i64 - new_size).abs() < delta {
                            optimal = epsilon as f64 / 1000.0;
                            delta = ((desired_size as i64) - new_size).abs();
                            polygon = Some(country);
                        }

                        if new_size < (desired_size as i64) {
                            break;
                        }
                    }

                    optimal_compression
                        .lock()
                        .unwrap()
                        .insert(country.0.clone(), (optimal, polygon));

                    eprintln!(
                        "Optimal compression for {}: {}",
                        country.0.country_name(),
                        optimal
                    );
                }
            }));
        });

    for handle in handles {
        handle.join().unwrap();
    }

    let optimal_compression = optimal_compression.lock().unwrap();
    fs::write(
        "assets/compressed.20kb.geojson",
        serde_json::to_string(
            &optimal_compression
                .iter()
                .map(|(code, (_, polygon))| {
                    (code, SerializableMultiPolygon(polygon.clone().unwrap()))
                })
                .collect::<HashMap<_, _>>(),
        )
        .unwrap(),
    )
    .unwrap();
}
