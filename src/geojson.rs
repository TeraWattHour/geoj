use std::{collections::HashMap, fs};

use strum::IntoEnumIterator;

use crate::{
    countries::CountryCode,
    shapes::SerializableMultiPolygon,
    simplification::{Simplification, SimplifiedBorders},
};

fn read_geojson(
    level: Simplification,
) -> Result<HashMap<CountryCode, geo::MultiPolygon>, anyhow::Error> {
    let geojson = fs::read_to_string(format!("assets/compressed.{}.geojson", level))?;
    let countries =
        serde_json::from_str::<HashMap<CountryCode, SerializableMultiPolygon>>(&geojson)?
            .iter()
            .map(|(k, v)| (k.clone(), v.0.clone()))
            .collect();
    Ok(countries)
}

fn read_geojson_blob() -> Result<[HashMap<CountryCode, geo::MultiPolygon>; 6], anyhow::Error> {
    let none = read_geojson(Simplification::None)?;
    let slight = read_geojson(Simplification::Slight)?;
    let medium = read_geojson(Simplification::Medium)?;
    let moderate = read_geojson(Simplification::Moderate)?;
    let aggressive = read_geojson(Simplification::Aggressive)?;
    let max = read_geojson(Simplification::Max)?;

    Ok([none, slight, medium, moderate, aggressive, max])
}

pub fn load_countries() -> Result<HashMap<CountryCode, Box<SimplifiedBorders>>, anyhow::Error> {
    let [none, slight, medium, moderate, aggressive, max] = read_geojson_blob()?;

    let mut variants = HashMap::new();

    let with_size = |poly: geo::MultiPolygon| {
        (
            serde_json::to_string(&SerializableMultiPolygon(poly.clone()))
                .unwrap()
                .len(),
            poly,
        )
    };

    for country in CountryCode::iter() {
        let borders = SimplifiedBorders {
            none: with_size(none.get(&country).unwrap().clone()),
            slight: with_size(slight.get(&country).unwrap().clone()),
            medium: with_size(medium.get(&country).unwrap().clone()),
            moderate: with_size(moderate.get(&country).unwrap().clone()),
            aggressive: with_size(aggressive.get(&country).unwrap().clone()),
            max: with_size(max.get(&country).unwrap().clone()),
        };

        variants.insert(country, Box::new(borders));
    }

    Ok(variants)
}
