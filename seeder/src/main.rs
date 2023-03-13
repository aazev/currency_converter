use std::{env, thread};

use chrono::{Duration, NaiveTime, Timelike, Utc};
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Change this to adjust the available requests per month
    let available_requests_per_month = env::var("API_MONTHLY_LIMIT")
        .unwrap_or("250".to_string())
        .parse::<i64>()
        .unwrap();
    let work_hours_per_day = 8; // Change this to adjust the number of work hours per day
    let interval_between_requests_in_minutes = 60; // Change this to adjust the interval between requests in minutes

    let requests_per_hour = available_requests_per_month / (work_hours_per_day * 30);
    let work_hours_per_day = if requests_per_hour > 0 {
        available_requests_per_month / (requests_per_hour * 30)
    } else {
        0
    };
    let work_hours_start_time = NaiveTime::from_hms_opt(9, 0, 0).unwrap(); // Change this to adjust the start time of the work hours
    let work_hours_end_time = work_hours_start_time + Duration::hours(work_hours_per_day);

    println!("Maximum requests per hour: {}", requests_per_hour);
    println!("Work hours per day: {}", work_hours_per_day);
    println!("Work hours start time: {}:00", work_hours_start_time);
    println!("Work hours end time: {}:00", work_hours_end_time);

    let mut last_request_time = Utc::now();
    let interval_between_requests = Duration::minutes(interval_between_requests_in_minutes);

    loop {
        let current_time = Utc::now();
        // let current_time = DateTime::<Utc>::from_utc(
        //     NaiveDateTime::new(
        //         NaiveDate::from_ymd_opt(2023, 03, 13).unwrap(),
        //         NaiveTime::from_hms_opt(11, 45, 49).unwrap(),
        //     ),
        //     Utc,
        // );

        if current_time.time().hour() < work_hours_start_time.hour()
            && current_time.time() - work_hours_start_time > Duration::seconds(0)
        {
            // If the current time is outside the work hours, wait until the start of the next work hour
            let sleep_time = current_time.time() - work_hours_start_time;
            println!("before Sleeping for {} seconds", sleep_time.num_seconds());
            thread::sleep(sleep_time.to_std().unwrap());
        } else if current_time.time().hour() > work_hours_end_time.hour()
            && work_hours_end_time - current_time.time() > Duration::seconds(0)
        {
            let sleep_time = work_hours_end_time - current_time.time();
            println!("after Sleeping for {} seconds", sleep_time.num_seconds());
            thread::sleep(sleep_time.to_std().unwrap());
        } else {
            // If the current time is within the work hours, check if the maximum number of requests allowed per hour has been reached
            let elapsed_time_since_last_request = Utc::now() - last_request_time;
            if elapsed_time_since_last_request < interval_between_requests {
                // If the minimum interval between requests has not been reached, wait until the interval has passed
                let sleep_time = interval_between_requests - elapsed_time_since_last_request;
                println!(
                    "Sleeping for {} milliseconds, since last request was only made {} seconds ago",
                    sleep_time.num_milliseconds(),
                    elapsed_time_since_last_request.num_seconds()
                );
                thread::sleep(sleep_time.to_std().unwrap());
            } else {
                // If the minimum interval between requests has been reached, make the request and record the time
                println!(
                    "Work hours, making request since last one was made {} seconds ago",
                    elapsed_time_since_last_request.num_seconds()
                );
                make_request();
                last_request_time = current_time;
            }
        }
    }
}

fn make_request() {
    // Make the API request here
    println!("API request made at {}", Utc::now());
}
