use std::collections::HashMap;

use axum::{extract::State, Json};
use serde::Serialize;

use crate::{countries::CountryCode, AppState};

#[derive(Serialize)]
pub struct CountriesResponse {
    name: String,
    sizes: (usize, usize, usize, usize, usize, usize),
}

pub async fn get_countries(
    State(state): State<AppState>,
) -> Result<Json<HashMap<CountryCode, CountriesResponse>>, ()> {
    let mut codes = HashMap::new();

    for (key, borders) in state.countries.lock().expect("Mutex poisoned").iter() {
        codes.insert(
            key.clone(),
            CountriesResponse {
                name: key.country_name().to_string(),
                sizes: borders.sizes(),
            },
        );
    }

    Ok(Json(codes))
}
