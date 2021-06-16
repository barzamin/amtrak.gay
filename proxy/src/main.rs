#[macro_use]
extern crate rocket;
use std::time::Duration;

use rocket::response::Debug;
use rocket::serde::json::{json, Value};
use rocket::State;
use geojson::GeoJson;
use thiserror::Error;

const ENDPOINT_GET_TRAINS: &str =
    "https://maps.amtrak.com/services/MapDataService/trains/getTrainsData";

#[derive(Debug, Error)]
enum AppErr {
    #[error("request to Amtrak api failed")]
    AmtrakRequestError(reqwest::Error),

    #[error("failed to decrypt Amtrak response")]
    DecryptionError(amtk::DecryptionError),

    #[error("couldn't parse response JSON after decryption")]
    ResponseParseFailed(geojson::Error),
}

#[get("/trains")]
async fn trains(rcl: &State<reqwest::Client>) -> Result<Value, Debug<AppErr>> {
    let resp = rcl
        .get(ENDPOINT_GET_TRAINS)
        .send()
        .await
        .map_err(AppErr::AmtrakRequestError)?
        .text().await.map_err(AppErr::AmtrakRequestError)?;

    let decrypted = amtk::decrypt(&resp).map_err(AppErr::DecryptionError)?;
    let geojson = decrypted.parse::<GeoJson>().map_err(AppErr::ResponseParseFailed)?;

    Ok(json!({
        "doggy": "woof",
        "geojson": geojson,
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(reqwest::Client::new())
        .mount("/", routes![trains])
}
