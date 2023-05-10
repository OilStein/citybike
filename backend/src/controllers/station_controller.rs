use actix_web::web::Data;
use surrealdb::{engine::local::Db, Surreal};

use crate::{models::station::Station, prelude::*};

pub struct StationBMC;

impl StationBMC {
    pub async fn get_all(db: Data<Surreal<Db>>) -> Result<Vec<Station>, Error> {
        let mut response = db.query("SELECT * FROM station LIMIT 10").await?;
        let stations: Vec<Station> = response.take(0)?;
        Ok(stations)
    }
}
