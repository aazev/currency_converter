use std::{env, ops::Sub, thread};

use chrono::{DateTime, Duration, NaiveTime, Timelike, Utc};
use common::ampq_requests::{RabbitMQRequest, RabbitMQRequestType};
use deadpool::managed::Object;
use deadpool_lapin::{Manager, Pool};
use dotenv::dotenv;
use lapin::{options::BasicPublishOptions, BasicProperties, ConnectionProperties};

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Change this to adjust the available requests per month
    let available_requests_per_month = env::var("API_MONTHLY_LIMIT")
        .unwrap_or("250".to_string())
        .parse::<i64>()
        .unwrap();
    let work_hours_per_day = 8; // Change this to adjust the number of work hours per day
    let interval_between_requests_in_minutes = 30; // Change this to adjust the interval between requests in minutes

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

    let mut last_request_time = round_time(Utc::now().sub(Duration::hours(1)));
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
                let _ = post_to_rabbitmq_queue().await;
                last_request_time = current_time;
            }
        }
    }
}

async fn post_to_rabbitmq_queue() {
    let ampq_addr: String =
        env::var("QUEUE_HOST").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());
    let queue_name = std::env::var("RMQ_QUEUE_NAME").unwrap_or_else(|_| "currency-fetcher".into());
    let manager: Manager = Manager::new(&ampq_addr, ConnectionProperties::default());

    let pool: Pool = deadpool::managed::Pool::builder(manager)
        .max_size(num_cpus::get())
        .build()
        .expect("can create pool");
    let rmq_con: Object<Manager> = pool
        .get()
        .await
        .map_err(|e| {
            eprintln!("could not get rmq con: {}", e);
            e
        })
        .unwrap();
    let rabbit_mq_channel = rmq_con.create_channel().await.unwrap();
    /*
    {
        "date_query": "2023-03-10T12:00:00",
        "date_start": "2022-01-01",
        "date_end": "2022-12-31",
        "base_symbol":"EUR",
        "request_type":"Fluctuations"
    }
    */
    let date_query = round_time(Utc::now());
    let payload = RabbitMQRequest {
        date_query: date_query.naive_utc(),
        date_start: Some(date_query.sub(Duration::days(1)).date_naive()),
        date_end: Some(date_query.date_naive()),
        base_symbol: Some("EUR".to_string()),
        request_type: RabbitMQRequestType::Fluctuations,
    };
    println!("payload: {:?}", &payload);
    let payload = serde_json::to_string(&payload).unwrap();
    let _ = rabbit_mq_channel
        .basic_publish(
            "",
            &queue_name,
            BasicPublishOptions::default(),
            &payload.as_bytes(),
            BasicProperties::default(),
        )
        .await;
}

fn round_time(dt: DateTime<Utc>) -> DateTime<Utc> {
    let minute = dt.minute();
    let new_minute = ((minute + 2) / 5) * 5; // Round the minute to the nearest multiple of 5
    let rounded_time = NaiveTime::from_hms_opt(dt.hour(), new_minute, 0).unwrap();
    let date = dt.date_naive();
    let rounded_datetime = date.and_time(rounded_time);
    DateTime::<Utc>::from_utc(rounded_datetime, Utc)
}
