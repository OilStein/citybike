use actix_web::web::Data;
use surrealdb::{engine::local::Db, Surreal};

use crate::{error::MyError, models::station::{RecordStation, SingleStationView}, prelude::*};
pub struct StationBMC;

impl StationBMC {
    pub async fn get_all_stations(db: Data<Surreal<Db>>) -> Result<Vec<RecordStation>, Error> {
        let mut response = db
            .query("SELECT * FROM station ORDER BY name_fi ASC")
            .await?;
        let stations: Vec<RecordStation> = response.take(0)?;
        Ok(stations)
    }

    pub async fn get_stations_page(
        db: Data<Surreal<Db>>,
        page: usize,
    ) -> Result<Vec<RecordStation>, Error> {
        let mut response = db
            .query(format!(
                "SELECT * FROM station ORDER BY name_fi ASC LIMIT 25 START {}",
                (page * 25)
            ))
            .await?;
        let stations: Vec<RecordStation> = response.take(0)?;
        Ok(stations)
    }

    pub async fn get_station_by_id(
        db: Data<Surreal<Db>>,
        id: &str,
    ) -> Result<SingleStationView, Error> {
        let mut response = db.query(
            format!("SELECT *, 
            array::pop(
                (SELECT count(dep_station_id == $parent.fid) AS starting, 
                        count(tar_station_id == $parent.fid) AS ending                      
                        FROM journey GROUP ALL)) as data 
                FROM station:{}", id))
            .await?;
        let station: Option<SingleStationView> = response.take(0)?;
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