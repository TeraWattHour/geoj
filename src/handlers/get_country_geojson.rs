use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;

use crate::{
    countries::CountryCode, shapes::SerializableMultiPolygon, simplification::Simplification,
    AppState,
};

#[derive(Deserialize)]
pub struct GetCountryQuery {
    compression: Option<Simplification>,
}

pub async fn get_country_geojson(
    State(state): State<AppState>,
    Path(country): Path<CountryCode>,
    Query(query): Query<GetCountryQuery>,
) -> Result<Json<SerializableMultiPolygon>, ()> {
    let compression = query.compression.unwrap_or(Simplification::None);
    let countries = state.countries.lock().unwrap();
    let variants = countries.get(&country).unwrap();

    Ok(Json(SerializableMultiPolygon(match compression {
        Simplification::None => variants.none.1.clone(),
        Simplification::Slight => variants.slight.1.clone(),
        Simplification::Medium => variants.medium.1.clone(),
        Simplification::Moderate => variants.moderate.1.clone(),
        Simplification::Aggressive => variants.aggressive.1.clone(),
        Simplification::Max => variants.max.1.clone(),
    })))
}
