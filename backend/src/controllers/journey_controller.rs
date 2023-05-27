use actix_web::web::Data;
use serde::{Deserialize, Serialize};
use surrealdb::{engine::local::Db, sql::Thing, Surreal};

use crate::prelude::*;

pub struct JourneyBMC;

impl JourneyBMC {
    pub async fn get_all_journeys(db: Data<Surreal<Db>>) -> Result<Vec<RecordJourney>, Error> {
        let mut response = db.query("SELECT * FROM journey LIMIT 10").await?;
        let journeys: Vec<RecordJourney> = response.take(0)?;
        Ok(journeys)
    }

    pub async fn get_journeys_pagination(db: Data<Surreal<Db>>, page: usize, order: &str) -> Result<Vec<RecordJourney>, Error> {
        let mut response = db.query(format!("SELECT * FROM journey ORDER BY {} ASC LIMIT 25 START {}", order , (page * 25))).await?;
        let journeys: Vec<RecordJourney> = response.take(0)?;
        Ok(journeys)

    }

    pub async fn get_journeys_search_by_stations(db: Data<Surreal<Db>>, query: &str) -> Result<Vec<RecordJourney>, Error> {
        let mut response = db.query(format!("SELECT * FROM journey WHERE dep_station_name = /(?i){}/ LIMIT 5", query)).await?;
        let journeys: Vec<RecordJourney> = response.take(0)?;
        Ok(journeys)
    }

    pub async fn get_journey_by_id(
        db: Data<Surreal<Db>>,
        id: &str,
    ) -> Result<RecordJourney, Error> {
        let journey: Option<RecordJourney> = db.select(("journey", id)).await?;
        Ok(journey.unwrap())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecordJourney {
    id: Thing,
    dep_station_name: String,
    tar_station_name: String,
    distance: usize,
    duration: usize,
}
