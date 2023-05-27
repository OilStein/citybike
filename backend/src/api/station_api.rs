use actix_web::{
    get,
    web::{Data, Path, Query},
    HttpResponse,
};
use log::info;
use serde::Deserialize;
use surrealdb::{engine::local::Db, Surreal};

use crate::{controllers::station_controller::StationBMC};

/*
#[get("/stations")]
pub async fn get_all_stations(db: Data<Surreal<Db>>) -> HttpResponse {
    let result = StationBMC::get_all_stations(db).await;
    match result {
        Ok(stations) => HttpResponse::Ok().json(stations),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
*/

#[derive(Debug, Deserialize)]
pub struct PageRequest {
    page: Option<usize>
}

#[get("/stations")]
pub async fn get_stations_by_page(db: Data<Surreal<Db>>, query: Query<PageRequest>) -> HttpResponse {
    let page_number = match query.page {
        Some(num) => num,
        None => 0
    };

    let result = StationBMC::get_stations_page(db, page_number).await;
    match result {
        Ok(stations) => HttpResponse::Ok().json(stations),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/stations/{id}")]
pub async fn get_station_by_id(db: Data<Surreal<Db>>, path: Path<String>) -> HttpResponse {
    let id: String = path.into_inner();
    info!("{}", id);
    let result = StationBMC::get_station_by_id(db, &id).await;
    match result {
        Ok(station) => HttpResponse::Ok().json(station),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
