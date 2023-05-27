use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse,
};
use surrealdb::{engine::local::Db, Surreal};

use crate::controllers::journey_controller::JourneyBMC;

#[get("/journeys")]
pub async fn get_all_journeys(db: Data<Surreal<Db>>) -> HttpResponse {
    let response = JourneyBMC::get_all_journeys(db).await;
    match response {
        Ok(journeys) => HttpResponse::Ok().json(journeys),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
/*
pub async fn get_journeys_page(db: Data<Surreal<Db>>) -> HttpResponse {
    let response = JourneyBMC::get::get_
}
*/
#[get("journeys/{id}")]
pub async fn get_journey_by_id(db: Data<Surreal<Db>>, id: Path<String>) -> HttpResponse {
    let response = JourneyBMC::get_journey_by_id(db, &id.into_inner()).await;
    match response {
        Ok(journey) => HttpResponse::Ok().json(journey),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
