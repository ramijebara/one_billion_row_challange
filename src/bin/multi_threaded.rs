use color_eyre::eyre::Result;
use dashmap::DashMap;
use log::info;
use rayon::prelude::*;
use std::{env, fs};

fn main() -> Result<()> {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    let filename = &args[1].to_owned();
    info!("opening filename: {}", filename);

    let data_string = fs::read_to_string(filename)?;

    let cities: DashMap<String, WeatherSummary> = DashMap::new();

    data_string
        .as_parallel_string()
        .par_lines()
        .for_each(|line| {
            if let Some((city, measurement)) = line.split_once(';') {
                if let Ok(temp) = measurement.parse::<f64>() {
                    cities
                        .entry(city.to_string())
                        .and_modify(|city_stats| {
                            city_stats.t_min = temp.min(city_stats.t_min);
                            city_stats.t_max = temp.max(city_stats.t_max);
                            city_stats.t_sum += temp;
                            city_stats.t_count += 1.0;
                        })
                        .or_insert(WeatherSummary {
                            t_min: temp,
                            t_max: temp,
                            t_sum: temp,
                            t_count: 1.0,
                        });
                }
            }
        });

    info!("Number of cities in hashtable: {}", cities.len());

    let mut sorted_cities: Vec<(String, WeatherSummary)> = cities.into_iter().collect();
    sorted_cities.sort_by(|a, b| a.0.cmp(&b.0));

    let sorted_cities_length = sorted_cities.len() - 1;

    print!("{{");
    sorted_cities
        .into_iter()
        .enumerate()
        .for_each(|(e, (city, stats))| {
            let mean = ((stats.t_sum / stats.t_count) * 10.0).ceil() / 10.0;
            print!("{}={:.1}/{:.1}/{:.1}", city, stats.t_min, mean, stats.t_max);
            if e < sorted_cities_length {
                print!(", ");
            }
        });
    println!("}}");

    Ok(())
}

#[derive(Debug, Default)]
struct WeatherSummary {
    t_min: f64,
    t_max: f64,
    t_sum: f64,
    t_count: f64,
}
