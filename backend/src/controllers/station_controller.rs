use actix_web::web::Data;
use surrealdb::{
    engine::local::Db,
    sql::{Array, Object, Value},
    Surreal,
};

use crate::{models::station::Station, prelude::*};

pub struct StationBMC;

impl StationBMC {
    pub async fn get_all(db: Data<Surreal<Db>>) -> Result<Vec<Object>, Error> {
        let mut response = db.query("SELECT * FROM station LIMIT 10").await?;
        let stations: Vec<Station> = response.take(0)?;
        let values: Vec<Value> = stations.into_iter().map(|x| Value::from(x)).collect();
        let array: Array = Array::try_from(values).expect("sda");
        array.into_iter().map(|x| W(x).try_into()).collect()
    }
}
