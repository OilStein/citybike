use actix_web::web::Data;
use surrealdb::{engine::local::Db, sql::Object, Surreal};

use crate::{models::station::Station, prelude::*};

pub struct StationBMC;

impl StationBMC {
    pub async fn get_all(db: Data<Surreal<Db>>) -> Result<Vec<Object>, Error> {
        let response: Vec<Object> = db.select("station").await?;
        Ok(response)
    }
}
