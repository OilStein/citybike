use actix_web::{get, web::Data, HttpResponse};
use surrealdb::{engine::local::Db, Surreal};

use crate::controllers::station_controller::StationBMC;

#[get("/stations")]
pub async fn get_stations(db: Data<Surreal<Db>>) -> HttpResponse {
    let result = StationBMC::get_all(db).await;
    match result {
        Ok(stations) => HttpResponse::Ok().json(stations),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
