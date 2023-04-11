use core::panic;
use std::{io::{self, Write}, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    println!("|----- WELCOME TO TIMELY -----|");
    print!("Enter the times you want to sum up: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)?;

    print!("Enter the speed times you wish to watch the videos in: ");
    io::stdout().flush().unwrap();

    let mut speed_times = String::new();
    io::stdin().read_line(&mut speed_times)?;

    process_times(input, speed_times);

    Ok(())
}

fn process_times(times: String, speed_times: String) {
    let mut times = times.split_whitespace();

    let mut some_time = times.next();
    let mut total_time: u32 = 0;

    loop {
        if let Some(time) = some_time {
            let hr_min: Vec<&str> = time.split(":").collect();
            if hr_min.len() != 2 {
                panic!("You need to use the hh:mm 24hr format.");
            }
            let hr: u32 = hr_min[0].parse().expect("Please use numbers");
            let min: u32 = hr_min[1].parse().expect("Please use numbers");
            if min >= 60 {
                panic!("Minutes must be less than 60");
            }
            total_time += hr * 60 + min;
            some_time = times.next();
        } else {
            break;
        }
    }

    for speed_time in speed_times.split_whitespace() {
        let speed_time: u32 = speed_time.parse().expect("Please use numbers");
        let hr = (total_time / speed_time) / 60;
        let min = (total_time / speed_time) % 60;
        
        if min < 10 {
            let mut min_str = String::from("0");
            min_str.push_str(&min.to_string());
            println!("\nYou can watch the videos in {}x speed in {}:{}.", speed_time, hr, min_str);
        } else {
            println!("\nYou can watch the videos in {}x speed in {}:{}.", speed_time, hr, min);
        }
    }
}