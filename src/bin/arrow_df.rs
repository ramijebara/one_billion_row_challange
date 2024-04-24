use color_eyre::eyre::Result;
use core::panic;
use datafusion::arrow::util::pretty;
use datafusion::prelude::*;
use datafusion::{
    arrow::datatypes::{Field, Schema},
    execution::{context::SessionContext, options::CsvReadOptions},
};
use log::info;
use std::env;
use tokio::runtime::Runtime;

fn main() -> Result<()> {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let filename = args
        .into_iter()
        .nth(1)
        .unwrap_or_else(|| panic!("you need to provide an input filename"));

    info!("filename: {}", filename);

    let runtime = Runtime::new()?;
    let context = SessionContext::new();

    let station_field = Field::new(
        "station",
        datafusion::arrow::datatypes::DataType::Utf8,
        false,
    );
    let temperature_field = Field::new(
        "temperature",
        datafusion::arrow::datatypes::DataType::Float64,
        false,
    );

    let schema = Schema::new(vec![station_field, temperature_field]);

    let csv_reader_options = CsvReadOptions::new()
        .delimiter(b';')
        .has_header(false)
        .file_extension("txt")
        .schema(&schema);

    let data_file = runtime.block_on(context.read_csv(filename, csv_reader_options))?;

    let results_future = data_file
        .aggregate(
            vec![col("station")],
            vec![
                min(col("temperature").alias("min_temp")),
                avg(col("temperature").alias("avg_temp")),
                max(col("temperature").alias("max_temp")),
            ],
        )?
        .sort(vec![col("station").sort(true, false)])?
        .collect();

    let results = runtime.block_on(results_future);

    let results_txt = pretty::pretty_format_batches(&results?)?;

    println!("{}", results_txt);

    Ok(())
}
