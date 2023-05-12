use actix_web::web::Data;
use surrealdb::{engine::local::Db, Surreal};

use crate::{error::MyError, models::station::RecordStation, prelude::*};
pub struct StationBMC;

impl StationBMC {
    pub async fn get_all_stations(db: Data<Surreal<Db>>) -> Result<Vec<RecordStation>, Error> {
        let mut response = db.query("SELECT * FROM station LIMIT 10").await?;
        let stations: Vec<RecordStation> = response.take(0)?;
        Ok(stations)
    }

    pub async fn get_station_by_id(
        db: Data<Surreal<Db>>,
        id: &str,
    ) -> Result<RecordStation, Error> {
        let mut response = db.query(format!("SELECT * FROM station:{}", id)).await?;
        let station: Option<RecordStation> = response.take(0)?;
        match station {
            Some(x) => {
                return Ok(x);
            }
            None => {
                return Err(Error::MyError(MyError::new(
                    "Station not found by id".to_string(),
                )));
            } // Ok(station.unwrap())
        }
    }
}
