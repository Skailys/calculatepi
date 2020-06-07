use chrono::prelude::*;
use std::time::SystemTime;

fn get_pi() -> &'static str {
    return "3.1415926535897932384626433832795028841971693993751058209749445923078164062862089986280348253421170679";
}

fn how_close_is_the_result(result: String) -> usize {

    //Get a char vec of result
    let result_as_vec: Vec<char> = result.chars().collect();

    //Get pi as vec of chars
    let pi_as_vec: Vec<char> = get_pi().chars().collect();

    //Create a counter
    let mut counter: usize = 0;

    //Compare
    loop {
        if result_as_vec[counter + 1].eq(&pi_as_vec[counter + 1]) {
            counter = counter + 1;
        } else {return counter}
    }
}

//TODO Add two difference modes

fn main() {

    //Get the start time
    let start_time: DateTime<Local> = Local::now();
    println!("The system time is {}!", start_time.time().to_string());

    //Get start time from system
    let start_time_of_os = SystemTime::now();

    //Delicate vars
    let debug_mode = false;
    let mut pi = 0.0;
    let mut next_plus = true;
    let mut i_as_f64;
    let number_of_trails: usize = 10 * 2;
    let trails_as_f64 = number_of_trails as f64;

    //Numbers of calculating
    for i in 1..number_of_trails {

        //Get i as f64
        i_as_f64 = i as f64;

        //Get the timestamp
        let date_time: DateTime<Local> = Local::now();
        let time_as_string = date_time.time().to_string();

        //Check if i is odd number
        if i_as_f64 % 2.0 == 0.0 {
            continue;
        }

        //Match next_plus
        match next_plus {
            true => { pi = pi + (4.0 / i_as_f64) }
            false => { pi = pi - (4.0 / i_as_f64) }
        }

        //Change the boolean
        next_plus = !next_plus;
        if debug_mode {
            println!("[Debug] next_plus is {}", next_plus);
        }

        //Print pi
        let message = "𝜋 is calculating. The value is currently by".to_string();
        println!("[{time}] {message} {pi}", time = time_as_string, message = message, pi = pi);

        //Print the progress
        println!("[Info] {}% finished!", (i_as_f64 / trails_as_f64) * 100.0);

        //Print a empty line
        println!(" ");
    }

    println!("[Result] Finished! 𝜋 ≈ {}", pi);
    println!("[Info] Pi was calculated to {} decimal digits exactly", how_close_is_the_result(format!("{}", pi)) - 1);

    //Extension
    println!("[Info] The calculating take {} secs!", start_time_of_os.elapsed().expect("[Debug] The time doesn't exist :O").as_secs());
}
