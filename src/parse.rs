use std::path::Path;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use chrono::NaiveDateTime;

use crate::model::FsLibreLine;
use crate::model::TIMESTAMP_FORMAT;

const SEPARATOR :&str = "\t";

pub fn parse_file(file :&File) -> Vec<FsLibreLine> {
    let reader = BufReader::new(file);

    let mut result = Vec::new();
    let mut line_counter = 0;
    for line_result in reader.lines() {
        if line_result.is_err() {
            print!("could not read line: ");
            println!("{}", line_result.unwrap_err());
            continue;
        }
        let line = line_result.unwrap();
        let parse_result = parse_line(&line, &line_counter);
        let line_error = parse_result.1;
        if !line_error {
            result.push(parse_result.0);
        }
        line_counter += 1;
    }
    return result;
}

fn parse_line(line :&String, line_counter :&u32) -> (FsLibreLine, bool) {
    //println!("{}", line);
    let mut fs_libre_line = FsLibreLine::new();
    let mut line_error = false;
    let mut index :usize = 0;
    let mut err_msg :&str = "";

    let mut parse_result = parse_id(line, & mut fs_libre_line);
    if parse_result.is_ok() {
        index = parse_result.unwrap();
    } else {
        line_error = true;
        err_msg = parse_result.err().unwrap();
    }

    if !line_error {
        parse_result = parse_timestamp(line, &mut fs_libre_line, &index);
        if parse_result.is_ok() {
            index = parse_result.unwrap();
        } else {
            line_error = true;
            err_msg = parse_result.err().unwrap();
        }
    }

    if !line_error {
        parse_result = parse_integer(line, &mut fs_libre_line.line_type, &index, "could not parse: line type");
        if parse_result.is_ok() {
            index = parse_result.unwrap();
        } else {
            line_error = true;
            err_msg = parse_result.err().unwrap();
        }
    }

    if !line_error {
        parse_result = parse_integer(line, &mut fs_libre_line.gluco_hist, &index, "could not parse: prev gluco");
        if parse_result.is_ok() {
            index = parse_result.unwrap();
        } else {
            index += 1;
        }
    }

    if !line_error {
        parse_result = parse_integer(line, &mut fs_libre_line.gluco_scanned, &index, "could not parse: scanned gluco");
        if parse_result.is_ok() {
            index = parse_result.unwrap();
        } else {
            index += 1;
        }
    }

    if !line_error {
        parse_result = parse_integer(line, &mut fs_libre_line.fast_insulin, &index, "could not parse: fast insulin");
        if parse_result.is_ok() {
            index = parse_result.unwrap();
        } else {
            index += 1;
        }
    }

    if !line_error {
        parse_result = parse_integer(line, &mut fs_libre_line.fast_insulin_non_numeric, &index, "could not parse: non numeric fast insulin");
        if parse_result.is_ok() {
            index = parse_result.unwrap();
        } else {
            index += 1;
        }
    }

    if !line_error {
        parse_result = parse_integer(line, &mut fs_libre_line.fast_insulin_units, &index, "could not parse: fast insulin units");
        if parse_result.is_ok() {
            index = parse_result.unwrap();
        } else {
            index += 1;
        }
    }

    if !line_error {
        parse_result = parse_integer(line, &mut fs_libre_line.food, &index, "could not parse: food");
        if parse_result.is_ok() {
            index = parse_result.unwrap();
        } else {
            index += 1;
        }
    }

    if !line_error {
        parse_result = parse_integer(line, &mut fs_libre_line.food_non_numeric, &index, "could not parse: non numeric food");
        if parse_result.is_ok() {
            index = parse_result.unwrap();
        } else {
            index += 1;
        }
    }

    if !line_error {
        parse_result = parse_integer(line, &mut fs_libre_line.carbohydrate, &index, "could not parse: carbohydrate");
        if parse_result.is_ok() {
            index = parse_result.unwrap();
        } else {
            index += 1;
        }
    }

    if !line_error {
        parse_result = parse_integer(line, &mut fs_libre_line.slow_insulin, &index, "could not parse: slow insulin");
        if parse_result.is_ok() {
            index = parse_result.unwrap();
        } else {
            index += 1;
        }
    }

    if !line_error {
        parse_result = parse_integer(line, &mut fs_libre_line.slow_insulin_non_numeric, &index, "could not parse: non numeric slow insulin");
        if parse_result.is_ok() {
            index = parse_result.unwrap();
        } else {
            index += 1;
        }
    }

    if !line_error {
        parse_result = parse_integer(line, &mut fs_libre_line.slow_insulin_units, &index, "could not parse: slow insulin units");
        //if parse_result.is_ok() {
        //    index = parse_result.unwrap();
        //} else {
        //    index += 1;
        //}
        if parse_result.is_err() {
            // never mind
        }
    }

    if line_error && *line_counter > 2 {
        println!("error parsing line: {}", err_msg);
        print!("    ");
        println!("{}", line);
    }
    return (fs_libre_line, line_error);
}

fn parse_id(line :&String, fs_libre_line :&mut FsLibreLine) -> Result<usize, &'static str> {
    let find_result = line.find(SEPARATOR);
    return if find_result.is_some() {
        let index = find_result.unwrap();
        let id_str = &line[0..index];
        fs_libre_line.id.push_str(id_str);
        Result::Ok(index)
    } else {
        Result::Err("could not parse: id")
    }
}

fn parse_timestamp(line :&String, fs_libre_line :&mut FsLibreLine, start_index :&usize) -> Result<usize, &'static str> {
    let line_remainer = &line[*start_index +1 .. line.len()];
    let find_result = line_remainer.find(SEPARATOR);
    let err_msg = "could not parse: timestamp";
    return if find_result.is_some() {
        let index = find_result.unwrap();
        if index > 0 {
            let timestamp_str: &str = &line_remainer[0 .. index];
            let parse_result = NaiveDateTime::parse_from_str(timestamp_str, TIMESTAMP_FORMAT);
            if parse_result.is_ok() {
                fs_libre_line.timestamp = parse_result.unwrap();
                Result::Ok(*start_index +1 + index)
            } else {
                Result::Err(err_msg)
            }
        } else {
            Result::Err(err_msg)
        }
    } else {
        Result::Err(err_msg)
    }
}

fn parse_integer(line :&String, target :&mut u32, start_index :&usize, err_msg :&'static str) -> Result<usize, &'static str> {
    let line_remainer = &line[*start_index +1 .. line.len()];
    let find_result = line_remainer.find(SEPARATOR);
    return if find_result.is_some() {
        let index = find_result.unwrap();
        if index > 0 {
            let mut int_str = &line_remainer[0..index];
            let decimal_result = int_str.find(",");
            if decimal_result.is_some() {
                let decimal_index = decimal_result.unwrap();
                if decimal_index > 0 {
                    int_str = &line_remainer[0..decimal_index];
                }
            }
            let parse_result = int_str.parse::<u32>();
            if parse_result.is_ok() {
                *target = parse_result.unwrap();
                Result::Ok(*start_index +1 + index)
            } else {
                Result::Err(err_msg)
            }
        } else {
            Result::Err(err_msg)
        }
    } else {
        Result::Err(err_msg)
    }
}

#[allow(unused_must_use)]
#[allow(dead_code)]
pub fn debug_print_file(data :&Vec<FsLibreLine>) {
    let path = Path::new("debug.csv");
    let create_result = File::create(&path);
    if create_result.is_ok() {
        let mut file = create_result.unwrap();
        for line in data {
            file.write_all(&line.id.as_bytes());
            file.write_all(SEPARATOR.as_bytes());
            file.write_all(&line.timestamp.format(TIMESTAMP_FORMAT).to_string().as_bytes());
            file.write_all(SEPARATOR.as_bytes());
            file.write_all(&line.line_type.to_string().as_bytes());
            file.write_all(SEPARATOR.as_bytes());
            if line.gluco_hist > 0 {
                file.write_all(&line.gluco_hist.to_string().as_bytes());
            }
            file.write_all(SEPARATOR.as_bytes());
            if line.gluco_scanned > 0 {
                file.write_all(&line.gluco_scanned.to_string().as_bytes());
            }
            file.write_all(SEPARATOR.as_bytes());
            if line.fast_insulin > 0 {
                file.write_all(&line.fast_insulin.to_string().as_bytes());
            }
            file.write_all(SEPARATOR.as_bytes());
            if line.fast_insulin_non_numeric > 0 {
                file.write_all(&line.fast_insulin_non_numeric.to_string().as_bytes());
            }
            file.write_all(SEPARATOR.as_bytes());
            if line.fast_insulin_units > 0 {
                file.write_all(&line.fast_insulin_units.to_string().as_bytes());
            }
            file.write_all(SEPARATOR.as_bytes());
            if line.food > 0 {
                file.write_all(&line.food.to_string().as_bytes());
            }
            file.write_all(SEPARATOR.as_bytes());
            if line.food_non_numeric > 0 {
                file.write_all(&line.food_non_numeric.to_string().as_bytes());
            }
            file.write_all(SEPARATOR.as_bytes());
            if line.carbohydrate > 0 {
                file.write_all(&line.carbohydrate.to_string().as_bytes());
            }
            file.write_all(SEPARATOR.as_bytes());
            if line.slow_insulin > 0 {
                file.write_all(&line.slow_insulin.to_string().as_bytes());
            }
            file.write_all(SEPARATOR.as_bytes());
            if line.slow_insulin_non_numeric > 0 {
                file.write_all(&line.slow_insulin_non_numeric.to_string().as_bytes());
            }
            file.write_all(SEPARATOR.as_bytes());
            if line.slow_insulin_units > 0 {
                file.write_all(&line.slow_insulin_units.to_string().as_bytes());
            }
            file.write_all(SEPARATOR.as_bytes());
            file.write_all("\n".as_bytes());
        }
    } else {
        println!("could not create debug file: {}", create_result.err().unwrap());
    }
}
