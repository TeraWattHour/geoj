use axum::{
    extract::{Path, Query, State},
    Json,
};
use geo::{Area, Centroid, Contains};
use serde::Deserialize;

use crate::{
    countries::CountryCode, shapes::SerializableMultiPolygon, simplification::Simplification,
    AppState,
};

#[derive(Deserialize)]
pub struct GetCountryQuery {
    compression: Option<Simplification>,
    only_mainland: Option<bool>,
}

pub async fn get_country_geojson(
    State(state): State<AppState>,
    Path(country): Path<CountryCode>,
    Query(query): Query<GetCountryQuery>,
) -> Result<Json<(Option<(f64, f64)>, SerializableMultiPolygon)>, ()> {
    let compression = query.compression.unwrap_or(Simplification::None);
    let only_mainland = query.only_mainland.unwrap_or(false) && country.capital_coords().is_some();

    let countries = state.countries.lock().unwrap();
    let variants = countries.get(&country).unwrap();

    let multipolygon = match compression {
        Simplification::None => variants.none.1.clone(),
        Simplification::Slight => variants.slight.1.clone(),
        Simplification::Medium => variants.medium.1.clone(),
        Simplification::Moderate => variants.moderate.1.clone(),
        Simplification::Aggressive => variants.aggressive.1.clone(),
        Simplification::Max => variants.max.1.clone(),
    };

    let multipolygon = geo::MultiPolygon(if only_mainland {
        let capital = country.capital_coords().unwrap();

        let mainland = multipolygon
            .clone()
            .into_iter()
            .filter(|polygon| polygon.contains(&geo::Point::new(capital.0, capital.1)))
            .collect::<Vec<_>>();

        if mainland.is_empty() {
            vec![multipolygon
                .iter()
                .max_by(|a, b| {
                    a.unsigned_area()
                        .partial_cmp(&b.unsigned_area())
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .unwrap()
                .clone()]
        } else {
            mainland
        }
    } else {
        multipolygon.0
    });

    let centroid = multipolygon
        .clone()
        .centroid()
        .and_then(|p| Some((p.y(), p.x())));

    Ok(Json((centroid, SerializableMultiPolygon(multipolygon))))
}
