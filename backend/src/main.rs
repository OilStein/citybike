use scylla::{FromUserType, IntoTypedRows, IntoUserType, Session, SessionBuilder};
use std::{error::Error, time::Instant};

use backend::utils::csv_reader::read_files;

#[derive(Debug, IntoUserType, FromUserType)]
struct MyType {
    int_val: i16,
    text_val: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    // Scylla
    let uri = std::env::var("SCYLLA_URI").unwrap_or_else(|_| "127.0.0.1:9042".to_string());

    let session: Session = SessionBuilder::new().known_node(uri).build().await?;

    session
        .query("CREATE KEYSPACE IF NOT EXISTS ks WITH REPLICATION = {'class': 'SimpleStrategy', 'replication_factor' :1}", &[])
        .await?;

    session
        .query(
            "CREATE TABLE IF NOT EXISTS ks.t (int_val int primary key, text_val text)",
            &[],
        )
        .await?;

    let test_insert = MyType {
        int_val: 12,
        text_val: "testi".to_string(),
    };

    println!("line38");
    session
        .query(
            "CREATE TYPE IF NOT EXISTS ks.my_type (int_val int, text_val text)",
            &[],
        )
        .await?;

    println!("line45");

    session
        .query(
            "INSERT INTO ks.t (int_val, text_val) VALUES(?) IF NOT EXISTS",
            (test_insert,),
        )
        .await?;

    println!("line 49");

    if let Some(rows) = session.query("SELECT a FROM ks.t", &[]).await?.rows {
        for row in rows.into_typed::<(MyType,)>() {
            let (my_type_value,): (MyType,) = row?;
            println!("{:?}", (my_type_value,));
        }
    }

    // CSV
    read_files();

    let end = Instant::now();

    println!("Duration: {:?}", (end - start));
    println!("Hello scylla");
    Ok(())
}
