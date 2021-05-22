mod model;
mod parse;
mod plot;

use crate::model::FsLibreLine;

use std::env;
use std::fs::File;
use chrono::{Datelike, NaiveDateTime};
use std::ops::Add;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("which file to open ???");
        return;
    }
    let path = &args[1];
    let file_result = File::open(&path);
    if file_result.is_err() {
        println!("could not open file: '{}'!!!", &path);
        println!("{}", file_result.unwrap_err());
        return;
    }
    let file = file_result.unwrap();
    let mut data = parse::parse_file(&file);
    //parse::debug_print_file(&data);

    println!("sorting lines");
    data.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    println!("unifying timestamps");
    data = unify_timestamps(&data);

    let mut data_of_day = Vec::new();
    for line in data {
        if data_of_day.is_empty() {
            data_of_day.push(line);
        } else {
            let prev_line = data_of_day.get(data_of_day.len() - 1);
            if prev_line.unwrap().timestamp.day() == line.timestamp.day() {
                data_of_day.push(line);
            } else {
                plot_wrapper(&data_of_day);
                data_of_day.clear();
                data_of_day.push(line);
            }
        }
    }
    // plot last day
    plot_wrapper(&data_of_day);
}

fn plot_wrapper(data_of_day :&Vec<FsLibreLine>) {
    let title_result = build_title(&data_of_day);
    if title_result.is_ok() {
        let title = title_result.unwrap();
        let path = build_path(&title);
        println!("creating file {}", path);
        let plot_result = plot::plot(&data_of_day, path.as_str(), title.as_str());
        if plot_result.is_err() {
            println!("error creating plot!!!");
            println!("{}", plot_result.err().unwrap());
        }
    }
}

fn build_title(data_of_day :&Vec<FsLibreLine>) -> Result<String, ()> {
    let line_result = data_of_day.get(0);
    if line_result.is_some() {
        let line = line_result.unwrap();
        let date_str = NaiveDateTime::format(&line.timestamp, "%Y-%m-%d");
        Ok(date_str.to_string())
    } else {
        Err(())
    }
}

fn build_path(title :&str) -> String {
    let path = String::from(title);
    path.add(".png")
}

fn unify_timestamps(data :&Vec<FsLibreLine>) -> Vec<FsLibreLine> {
    let mut unified = Vec::new();
    let mut prev_line :&FsLibreLine = &FsLibreLine::new();
    for line in data {
        if line.timestamp == prev_line.timestamp {
            let mut unified_line = FsLibreLine::new();
            unified_line.timestamp = line.timestamp;
            unified_line.fast_insulin = if line.fast_insulin > 0 {line.fast_insulin} else {prev_line.fast_insulin};
            unified_line.fast_insulin_units = if line.fast_insulin_units > 0 {line.fast_insulin_units} else {prev_line.fast_insulin_units};
            unified_line.fast_insulin_non_numeric = if line.fast_insulin_non_numeric > 0 {line.fast_insulin_non_numeric} else {prev_line.fast_insulin_non_numeric};
            unified_line.food = if line.food > 0 {line.food} else {prev_line.food};
            unified_line.food_non_numeric = if line.food_non_numeric > 0 {line.food_non_numeric} else {prev_line.food_non_numeric};
            unified_line.carbohydrate = if line.carbohydrate > 0 {line.carbohydrate} else {prev_line.carbohydrate};
            unified_line.slow_insulin = if line.slow_insulin > 0 {line.slow_insulin} else {prev_line.slow_insulin};
            unified_line.slow_insulin_units = if line.slow_insulin_units > 0 {line.slow_insulin_units} else {prev_line.slow_insulin_units};
            unified_line.slow_insulin_non_numeric = if line.slow_insulin_non_numeric > 0 {line.slow_insulin_non_numeric} else {prev_line.slow_insulin_non_numeric};
            unified.push(unified_line);
        } else {
            unified.push(line.clone());
        }
        prev_line = line;
    }
    unified
}
