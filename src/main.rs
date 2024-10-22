mod compress;
mod countries;
mod geojson;
mod handlers;
mod shapes;
mod simplification;

use axum::{routing::get, Router};
use axum_response_cache::CacheLayer;
use countries::CountryCode;
use handlers::{get_countries::*, get_country_geojson::*};
use simplification::SimplifiedBorders;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tower_http::services::ServeFile;

#[derive(Clone)]
struct AppState {
    countries: Arc<Mutex<HashMap<CountryCode, Box<SimplifiedBorders>>>>,
}

#[tokio::main]
async fn main() {
    let app_state = AppState {
        countries: Arc::new(Mutex::new(geojson::load_countries().unwrap())),
    };

    let app = Router::new()
        .route("/geojson/:alpha3", get(get_country_geojson))
        .route("/countries", get(get_countries))
        .layer(CacheLayer::with_lifespan(3600))
        .nest_service("/visualize", ServeFile::new("assets/visualize.html"))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
