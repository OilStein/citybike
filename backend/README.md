# Citybike Backend

## Prequisites
- [Rust tools](https://www.rust-lang.org/tools/install)
- [Clang](https://clang.llvm.org/get_started.html) or [gcc](https://gcc.gnu.org/install/) - For RocksDB embedding

## Datasets - Optional
Project contains all stations data and 100 000 lines of journeys 

For the exercise download three datasets of journey data. The data is owned by City Bike Finland.

* <https://dev.hsl.fi/citybikes/od-trips-2021/2021-05.csv>
* <https://dev.hsl.fi/citybikes/od-trips-2021/2021-06.csv>
* <https://dev.hsl.fi/citybikes/od-trips-2021/2021-07.csv>

Also, there is a dataset that has information about Helsinki Region Transport’s (HSL) city bicycle stations.

* Dataset: <https://opendata.arcgis.com/datasets/726277c507ef4914b0aec3cbcfcbfafc_0.csv>
* License and information: <https://www.avoindata.fi/data/en/dataset/hsl-n-kaupunkipyoraasemat/resource/a23eef3a-cc40-4608-8aa2-c730d17e8902>

### Setting more dataset to project - Optional

Add csv files to data folder in the root.
Then add or uncomment a line from src/utils/csv_reader.rs
Written line should look like this, where "2021-07.csv" is the files name:

```
 send_journeys_to_db(db, read_journeys("2021-07.csv").await?).await?;
```
## Running application

To run backend:
```
<sub>Unoptimized</sub>
cargo run 
or
<sub>Optimized</sub>
cargo run --release 
```



