// Single threaded The One Billion Row Challenge in rust
// More info: https://github.com/gunnarmorling/1brc
use color_eyre::eyre::Result;
use log::info;
use std::{
    collections::HashMap, env, fs::File, io::{BufRead, BufReader}
};

fn main() -> Result<()> {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let filename = &args[1].to_owned();
    info!("opening filename: {}", filename);

    let file = File::open(filename)?;
    let mut data_lines = BufReader::new(file).lines();
    let mut cities: HashMap<String, WeatherSummary> = HashMap::new();

    while let Some(Ok(line)) = data_lines.next() {
        if let Some((city, measurement)) = line.split_once(';') {
            if let Ok(value) = measurement.parse::<f64>() {
                cities
                    .entry(city.to_string())
                    .and_modify(|city_stats| {
                        city_stats.min = value.min(city_stats.min);
                        city_stats.max = value.max(city_stats.max);
                        city_stats.sum += value;
                        city_stats.reading_count += 1.0;
                    })
                    .or_insert(WeatherSummary {
                        min: value,
                        max: value,
                        sum: value,
                        reading_count: 1.0,
                    });
            }
        }
    }

    info!("Number of cities in hashtable: {}", cities.len());

    let mut sorted_cities: Vec<(String, WeatherSummary)> = cities.into_iter().collect();
    sorted_cities.sort_by(|a, b| {a.0.cmp(&b.0)});
    
    sorted_cities.into_iter().for_each(|(city, stats)| {
        let mean = stats.sum/stats.reading_count;
        println!("{}={:.1}/{:.1}/{:.1}", city, stats.min, mean, stats.max);
    });

    Ok(())
}

#[derive(Debug, Default)]
struct WeatherSummary {
    min: f64,
    max: f64,
    sum: f64,
    reading_count: f64,
}
