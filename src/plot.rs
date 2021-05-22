use crate::model::FsLibreLine;

use std::ops::{Sub, Add};
use std::io::BufReader;
use std::fs::File;
use chrono::{Duration, NaiveTime, NaiveDateTime};
use plotters::prelude::*;
use image::{imageops::FilterType, ImageFormat};

pub fn plot(data_of_day :&Vec<FsLibreLine>, path :&str, title :&str) -> Result<(), Box<dyn std::error::Error>> {
    let y_min = -100;
    let y_max = 350;
    let target_range_min = 60;
    let target_range_max = 180;

    let backend = BitMapBackend::new(path, (800, 600));
    let root = backend.into_drawing_area();
    root.fill(&WHITE)?;

    let (from_date, to_date) = (
        to_duration(&data_of_day[0].timestamp),
        to_duration(&data_of_day[data_of_day.len() - 1].timestamp),
    );

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .caption(title, ("sans-serif", 30.0).into_font())
        .build_cartesian_2d(from_date..to_date, y_min..y_max)?;

    chart.configure_mesh()
        .light_line_style(&WHITE)
        .x_label_formatter(&|x| format!("{}:00", x.num_hours()))
        .y_label_formatter(&|y| format!("{}", y))
        .draw()?;

    // red area
    chart.plotting_area().draw(&Rectangle::new(
        [(zero_duration(), 0), (max_width_duration(), target_range_min)],
        RGBColor(255,179, 179).filled()))?;
    // target area
    chart.plotting_area().draw(&Rectangle::new(
        [(zero_duration(), target_range_min), (max_width_duration(), target_range_max)],
        RGBColor(173,216,230).filled()))?;

    let mut line_dots = Vec::new();
    chart.draw_series(
        data_of_day.iter()
            .map(|x| {
                let y :i32 = (if x.gluco_scanned > 0 {x.gluco_scanned} else {x.gluco_hist}) as i32;
                if y > 0 {
                    let style = if y > target_range_min {BLACK.filled()} else {RED.filled()};
                    let x = to_duration(&x.timestamp);
                    line_dots.push((x, y));
                    Circle::new((x, y), 5, style)
                } else {
                    Circle::new((Duration::seconds(0),0), 0, BLACK.filled())
                }
            }),
    )?;
    chart.draw_series(LineSeries::new(line_dots, &BLACK))?;

    let legend_y = -10;
    let legend_line_height = 2;
    let legend_text_height = 20;
    let legend_text_line_offset = 6;
    let legend_y_insulin_fast = legend_y - legend_line_height - 3;
    let legend_y_food = legend_y_insulin_fast - legend_text_height;
    let legend_y_insulin_slow = legend_y_food - legend_text_height;
    chart.plotting_area().draw(&Rectangle::new(
        [(zero_duration(), legend_y), (max_width_duration(), legend_y - legend_line_height)],
        BLACK.filled()))?;
    chart.plotting_area().draw(&Rectangle::new(
        [
            (zero_duration(), legend_y_food + legend_text_line_offset),
            (max_width_duration(), legend_y_food + legend_text_line_offset - legend_line_height / 2)
        ],
        BLACK.filled()))?;
    chart.plotting_area().draw(&Rectangle::new(
        [
            (zero_duration(), legend_y_insulin_slow + legend_text_line_offset),
            (max_width_duration(), legend_y_insulin_slow + legend_text_line_offset - legend_line_height / 2)
        ],
        BLACK.filled()))?;

    let font_default :FontDesc = ("sans-serif", 14).into_font();
    font_default.color(&BLACK);
    /*
    note: rendering icons as emoji-text has issues
    let font_icons :FontDesc = ("monospace", 10).into_font();
    font_icons.color(&BLACK);
    let font_icons_slow :FontDesc = ("monospace", 10).into_font();
    font_icons_slow.color(&GREEN);
    chart.plotting_area().draw(&Text::new("💉", (zero_duration(), legend_y_insulin_fast), font_icons.clone()))?;
    chart.plotting_area().draw(&Text::new("🍎", (zero_duration(), legend_y_food),font_icons.clone()))?;
    chart.plotting_area().draw(&Text::new("💉", (zero_duration(), legend_y_insulin_slow), font_icons_slow.clone()))?;
     */
    let image_syringe = image::load(BufReader::new(File::open(build_image_path_syringe())?),ImageFormat::Png)?.resize_exact(12, 12, FilterType::Nearest);
    let image_food = image::load(BufReader::new(File::open(build_image_path_food())?),ImageFormat::Png)?.resize_exact(12, 12, FilterType::Nearest);
    let image_syringe_slow = image::load(BufReader::new(File::open(build_image_path_syringe_slow())?),ImageFormat::Png)?.resize_exact(12, 12, FilterType::Nearest);
    let bitmap_syringe_legend : BitMapElement<_> = ((zero_duration(), legend_y_insulin_fast), image_syringe.clone()).into();
    let bitmap_food_legend : BitMapElement<_> = ((zero_duration(), legend_y_food), image_food.clone()).into();
    let bitmap_syringe_slow_legend : BitMapElement<_> = ((zero_duration(), legend_y_insulin_slow), image_syringe_slow.clone()).into();
    chart.draw_series(std::iter::once(bitmap_syringe_legend))?;
    chart.draw_series(std::iter::once(bitmap_food_legend))?;
    chart.draw_series(std::iter::once(bitmap_syringe_slow_legend))?;

    let info_icon_y_fast_insulin = 340;
    let info_icon_y_food = info_icon_y_fast_insulin - 12;
    let info_icon_y_slow_insulin = info_icon_y_food - 12;
    for x in data_of_day {
        let fast_insulin :i32 = (if x.fast_insulin_units > 0 {x.fast_insulin_units} else if x.fast_insulin > 0 {x.fast_insulin} else {x.fast_insulin_non_numeric}) as i32;
        let slow_insulin :i32 = (if x.slow_insulin_units > 0 {x.slow_insulin_units} else if x.slow_insulin > 0 {x.slow_insulin} else if x.slow_insulin_non_numeric > 0 {x.slow_insulin_non_numeric} else {x.carbohydrate}) as i32;
        let food :i32 = (if x.food > 0 {x.food} else if x.food_non_numeric > 0 {x.food_non_numeric} else {0}) as i32;

        if fast_insulin > 0 {
            //chart.plotting_area().draw(&Text::new("💉", (to_duration(&x.timestamp), info_icon_y_fast_insulin), font_icons.clone()))?;
            let bitmap_syringe_usage : BitMapElement<_> = ((to_duration(&x.timestamp), info_icon_y_fast_insulin), image_syringe.clone()).into();
            chart.draw_series(std::iter::once(bitmap_syringe_usage))?;
            chart.plotting_area().draw(&Text::new(fast_insulin.to_string(), (to_duration(&x.timestamp), legend_y_insulin_fast), font_default.clone()))?;
        }
        if food > 0 {
            //chart.plotting_area().draw(&Text::new("🍎", (to_duration(&x.timestamp), info_icon_y_food), font_icons.clone()))?;
            let bitmap_food_usage : BitMapElement<_> = ((to_duration(&x.timestamp), info_icon_y_food), image_food.clone()).into();
            chart.draw_series(std::iter::once(bitmap_food_usage))?;
            chart.plotting_area().draw(&Text::new(food.to_string(), (to_duration(&x.timestamp), legend_y_food), font_default.clone()))?;
        }
        if slow_insulin > 0 {
            //chart.plotting_area().draw(&Text::new("💉", (to_duration(&x.timestamp), info_icon_y_slow_insulin), font_icons_slow.clone()))?;
            let bitmap_syringe_slow_usage : BitMapElement<_> = ((to_duration(&x.timestamp), info_icon_y_slow_insulin), image_syringe_slow.clone()).into();
            chart.plotting_area().draw(&Text::new(slow_insulin.to_string(), (to_duration(&x.timestamp), legend_y_insulin_slow), font_default.clone()))?;
            chart.draw_series(std::iter::once(bitmap_syringe_slow_usage))?;
        }
        if fast_insulin > 0 || slow_insulin > 0 || food > 0 {
            chart.plotting_area().draw(&Rectangle::new(
                [(to_duration_offset(&x.timestamp), legend_y_insulin_slow - legend_text_height), (to_duration_offset(&x.timestamp), 400)],
                BLACK.filled()))?;
        }
    }

    Ok(())
}

fn to_duration(timestamp :&NaiveDateTime) -> Duration {
    timestamp.time().signed_duration_since(NaiveTime::from_hms(0, 0, 0))
}

fn to_duration_offset(timestamp :&NaiveDateTime) -> Duration {
    let mut time = timestamp.time();
    time = time.sub(Duration::minutes(5));
    time.signed_duration_since(NaiveTime::from_hms(0, 0, 0))
}

fn zero_duration() -> Duration {
    Duration::seconds(0)
}

fn max_width_duration() -> Duration {
    Duration::hours(24)
}

fn resources_dir() -> &'static str {
    return "res";
}

fn delim() -> &'static str {
    return "/";
}

fn build_image_path_syringe() -> String {
    let path = String::from(resources_dir()).add(delim()).add("syringe.png");
    path
}

fn build_image_path_syringe_slow() -> String {
    let path = String::from(resources_dir()).add(delim()).add("syringe_slow.png");
    path
}

fn build_image_path_food() -> String {
    let path = String::from(resources_dir()).add(delim()).add("apple.png");
    path
}
