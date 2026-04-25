#[cfg(target_os = "linux")]
const LKH_BINARY: &[u8] = include_bytes!("../resources/LKH");

#[cfg(target_os = "windows")]
const LKH_BINARY: &[u8] = include_bytes!("../resources/LKH.exe");

use dotenv::*;
use eframe::egui;
use polars::prelude::*;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

struct NodeCoords {
    id: usize,
    x: f64,
    y: f64,
    weight: Option<f64>,
}

struct TspData {
    name: String,
    comment: String,
    dimension: usize,
    edge_weight_type: String,
    node_coords: Vec<NodeCoords>,
}

/* We're just going to focus on the csv -> TSP pipeline, no XLSX for now */

fn ingest_csv(path: &Path) -> PolarsResult<LazyFrame> {
    let file_path = PlRefPath::try_from_path(path)?;
    let lf = LazyCsvReader::new(file_path)
        .with_has_header(true)
        .finish()?;
    Ok(lf)
}

fn geocoding_frame(df: DataFrame, lf: PolarsResult<LazyFrame>) -> PolarsResult<LazyFrame> {}

fn load_zipcode_data() -> PolarsResult<DataFrame> {
    let geocode_file = env::var("geo_code").expect("Please check the encoded env variables");
    let file_path = PathBuf::from(geocode_file);
    let file = File::open(&file_path)?;
    let df = CsvReader::new(file).finish();
    df
}

fn df_to_tsp(df: DataFrame) -> TspData {
    let df = df;

    return TspData;
}

/*
 * fn main() -> eframe::Result<()> {}
 * dotenv().ok();
*/
