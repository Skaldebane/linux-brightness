use std::env;
use std::fs::{read_to_string, write};
use std::io::ErrorKind;
use std::process::exit;

const BRIGHTNESS_PATH: &str = "/sys/class/backlight/intel_backlight/brightness";
const ACTUAL_PATH: &str = "/sys/class/backlight/intel_backlight/actual_brightness";
const MAX_PATH: &str = "/sys/class/backlight/intel_backlight/max_brightness";

fn main() {
    let args: Vec<String> = env::args().collect();

    let actual = read_to_num(ACTUAL_PATH);
    let max = read_to_num(MAX_PATH);

    if args.len() >= 4 {
        eprintln!("Error: Too many arguments!");
        exit(1)
    }

    match args.get(1) {
        None => {
            println!("Current brightness = {actual}");
            println!("Maximum brightness = {max}")
        }
        Some(arg) => {
            match arg.trim() {
                "+" | "-" => {
                    match args.get(2) {
                        None => {
                            let change = max / 16;
                            if arg.trim() == "+" {
                                set_brightness(actual, actual + change, max)
                            }
                            if arg.trim() == "-" {
                                set_brightness(actual, actual - change, max)
                            }
                        }
                        Some(value) => {
                            let change: i32 = match value.parse() {
                                Ok(number) => number,
                                Err(_) => {
                                    eprintln!("Error: {value} is not a valid number!");
                                    exit(1)
                                }
                            };
                            if arg.trim() == "+" {
                                set_brightness(actual, actual + change, max)
                            }
                            if arg.trim() == "-" {
                                set_brightness(actual, actual - change, max)
                            }
                        }
                    }
                }
                _ => {
                    let new_value: i32 = match arg.trim().parse() {
                        Ok(number) => number,
                        Err(_) => {
                            println!("Error: {arg} is not a valid brightness value!");
                            exit(1)
                        }
                    };
                    set_brightness(actual, new_value, max);
                }
            }
        }
    }
}

fn set_brightness(actual: i32, new: i32, max: i32) {
    if new >= max && actual >= max {
        println!("Already at max brightness value {max}");
        exit(0)
    }
    let new_value = if new >= max { max } else if new <= 0 { 0 } else { new };
    match write(BRIGHTNESS_PATH, new_value.to_string()) {
        Err(err) => {
            match err.kind() {
                ErrorKind::PermissionDenied => {
                    eprintln!("Error: Permission denied. Couldn't change the brightness!");
                    exit(1)
                }
                _ => {}
            }
        }
        _ => {
            println!("Changing brightness from {actual} to {new_value}...")
        }
    }
}

fn read_to_num(path: &str) -> i32 {
    return match read_to_string(path) {
        Ok(result) => match result.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                eprintln!("Error: Invalid value at {path}!");
                exit(1)
            }
        },
        Err(_) => {
            eprintln!("Error: Can't find '{path}'");
            exit(1)
        }
    };
}
