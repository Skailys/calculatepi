use chrono::prelude::*;
use colored::*;
use std::time::SystemTime;
use crate::CalculationMethode::{GregoryLeibniz, Nilkantha};

fn get_pi() -> &'static str {
    return "3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679";
}

struct Pi {
    the_correct_part: ColoredString,
    the_incorrect_part: ColoredString,
    number_of_correct_digits: usize,
}

fn how_close_is_the_result(result: f64) -> Pi {

    //Get a char vec of result
    let result_as_vec: Vec<char> = format!("{}", result).chars().collect();

    //Get pi as vec of chars
    let pi_as_vec: Vec<char> = get_pi().chars().collect();

    //Get the values for Pi
    let mut counter: usize = 0;
    let mut correct_part: Vec<char> = vec!();
    let mut incorrect_part: Vec<char> = vec!();

    //Compare
    loop {
        if result_as_vec[counter].eq(&pi_as_vec[counter]) {
            correct_part.push(result_as_vec[counter]);
            counter = counter + 1;
        } else { break }
    }

    //Get the incorrect part
    for i in (counter + 1)..result_as_vec.len() {
        incorrect_part.push(result_as_vec[i]);
    }

    let pi = Pi {
        the_correct_part: correct_part.into_iter().collect::<String>().green(),
        the_incorrect_part: incorrect_part.into_iter().collect::<String>().red(),
        number_of_correct_digits: counter,
    };

    return pi;
}

fn print_state(options: &Options, timestamp: &DateTime<Local>, pi: &f64, &i: &f64, next_plus: &bool) {

    //Print debug information if it is wanted
    if options.debug_mode {
        println!("[Debug] next_plus is {}", next_plus);
    }

    //Print pi
    let time_as_string = timestamp.time().to_string();
    let message = "𝜋 is calculating. The value is currently by".to_string();
    println!("[{time}] {message} {pi}", time = time_as_string, message = message, pi = pi);

    //Print the progress
    let trails_as_f64 = options.trails as f64;
    println!("[Info] {}% finished!", (i / trails_as_f64) * 100.0);

    //Print a empty line
    println!(" ");
}

fn gregory_leibniz(options: Options) -> f64 {
    let mut pi = 0.0;
    let mut i_as_f64: f64;
    let mut next_plus = true;

    //Numbers of calculating
    for i in 1..(options.trails) {

        //Get i as f64
        i_as_f64 = i as f64;

        //Check if i is odd number
        if i_as_f64 % 2.0 == 0.0 {
            continue;
        }

        //Get the timestamp
        let date_time: DateTime<Local> = Local::now();

        //Match next_plus
        match next_plus {
            true => { pi = pi + (4.0 / i_as_f64) }
            false => { pi = pi - (4.0 / i_as_f64) }
        }

        //Change the boolean
        next_plus = !next_plus;

        //Print the current state
        print_state(&options, &date_time, &pi, &i_as_f64, &next_plus)
    }

    return pi;
}

fn nilkantha(options: Options) -> f64 {
    let mut pi = 3.0;
    let trails_as_f64 = (options.trails) as f64;
    let mut next_plus = true;

    let mut i = 2.0;
    loop {

        //Exit the loop if 'i' is bigger as the number of trails
        if i >= trails_as_f64 {
            return pi;
        }

        //Get the timestamp
        let timestamp: DateTime<Local> = Local::now();

        //Match next_plus
        match next_plus {
            true => { pi = pi + 4.0 / (i * (i + 1.0) * (i + 2.0)) }
            false => { pi = pi - 4.0 / (i * (i + 1.0) * (i + 2.0)) }
        }

        //Negate next_plus
        next_plus = !next_plus;

        //Change i to i+2
        i = i + 2.0;

        print_state(&options, &timestamp, &pi, &i, &next_plus)
    }
}

enum CalculationMethode {
    GregoryLeibniz,
    Nilkantha,
}

struct Options {
    debug_mode: bool,
    trails: usize,
    calculation_method: CalculationMethode,
}

fn main() {

    //Get the start time
    let start_time: DateTime<Local> = Local::now();
    println!("The system time is {}!", start_time.time().to_string());

    //Get start time from system
    let start_time_of_os = SystemTime::now();

    //Delicate options
    let options = Options {
        debug_mode: true,
        trails: 1000,
        calculation_method: CalculationMethode::Nilkantha,
    };

    let pi;
    match options.calculation_method {
        GregoryLeibniz => pi = gregory_leibniz(options),
        Nilkantha => pi = nilkantha(options),
    }

    let scan_of_pi = how_close_is_the_result(pi);
    println!("[Result] Finished! 𝜋 ≈ {correct}{incorrect}", correct = scan_of_pi.the_correct_part, incorrect = scan_of_pi.the_incorrect_part);
    println!("[Info] Pi was calculated to {} decimal digits exactly", scan_of_pi.number_of_correct_digits);

    //Extension
    let duration = start_time_of_os.elapsed().expect("[Debug] The time doesn't exist!");
    if duration.as_secs() != 0 {
        println!("[Info] The calculating take {} secs!", duration.as_secs())
    } else {
        println!("[Info] The calculating take {} secs!", format!("{:.2}", duration.as_secs_f32()));
    }
}
