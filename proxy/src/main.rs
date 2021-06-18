#[macro_use]
extern crate rocket;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::response::Debug;
use rocket::serde::json::{json, Value};
use rocket::State;
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
    ResponseParseFailed(serde_json::Error),
}

struct CORS {}

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "CORS on all routes",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _req: &'r rocket::Request<'_>, res: &mut rocket::Response<'r>) {
        res.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        res.set_header(Header::new("Access-Control-Allow-Methods", "GET"));
    }
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
    let geojson = decrypted.parse::<Value>().map_err(AppErr::ResponseParseFailed)?;

    Ok(json!({
        "puppy": "wuf", // 🐶
        "geojson": geojson,
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(CORS {})
        .manage(reqwest::Client::new())
        .mount("/", routes![trains])
}
