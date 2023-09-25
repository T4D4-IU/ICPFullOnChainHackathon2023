#[macro_use] extern crate rocket;

use std::fs;
use reqwest;

#[post("/snapshot", data = "<url>")]
async fn create_snapshot(url: String) -> Result<(), rocket::response::status::Custom<String>> {
    let html = fetch_html(&url).await.map_err(|e| rocket::response::status::Custom(rocket::http::Status::InternalServerError, e.to_string()))?;
    save_html("/src/ICPFullOnChainHackathon2023_frontend/src/archived/", &html).map_err(|e| rocket::response::status::Custom(rocket::http::Status::InternalServerError, e.to_string()))?;
    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![create_snapshot])
}

async fn fetch_html(url: &str) -> Result<String, reqwest::Error> {
    let resp = reqwest::get(url).await?;
    let body = resp.text().await?;
    Ok(body)
}

fn save_html(path: &str, html: &str) -> std::io::Result<()> {
    fs::write(path, html)
}