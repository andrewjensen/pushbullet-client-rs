extern crate pshbullet_client;
extern crate chrono;
extern crate dotenv;
extern crate simple_logger;

use std::env;
use dotenv::dotenv;
use chrono::prelude::*;
use pshbullet_client::*;
use pshbullet_client::push::*;


fn get_config(key: &str) -> String {
    let error = format!("couldn't find required environment variable {}", key);
    env::var(key).expect(&error)
}

fn main() {
    simple_logger::init().unwrap();
    dotenv().ok();
    let access_token = get_config("PUSHBULLET_TOKEN");

    let client = PushbulletClient::new(access_token);
    let mut condition = ListCondition::new(5);
    condition.set_modified_after(Utc::now() - chrono::Duration::weeks(1));

    let result = client.list_push(&condition);
    match result {
        Ok((responses, headers)) => {
            println!("number of results: {}", responses.len());
            println!("response_headers:");
            println!("  ratelimit_limit: {}", headers.ratelimit_limit.unwrap_or(0));
            println!("  ratelimit_remaining: {}", headers.ratelimit_remaining.unwrap_or(0));
            println!("  ratelimit_reset (UTC): {}",
                     headers.ratelimit_reset_time().map(|t| t.to_rfc3339())
                         .unwrap_or_else(|| "".to_owned()));

            println!("results: ");
            for p in &responses {
                println!("{:?}", p);
            }
        }
        Err(err) => println!("error: {}", err)
    }

    std::process::exit(0)
}
