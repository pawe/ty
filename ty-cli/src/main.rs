extern crate clap;
use clap::{Arg, App};
use reqwest;
use dotenv::dotenv;

use ty_lib::ThankYouMessage;

fn main() {
    dotenv().ok();

    let matches = App::new("ty - thank you")
        .version("0.1")
        .author("Paul Weißenbach <paul.weissenbach@aon.at>")
        .about("Say thank you to the tools (and hopefully it's authors) you use by simply typing ty in your terminal.")
        .arg(Arg::with_name("TOOL")
            .help("Name of the tool you want to thank. If left blank, it takes the last command in the history.")
            .required(true)
            .index(1))
        .arg(Arg::with_name("message")
            .short("m")
            .long("message")
            .takes_value(true)
            .multiple(false)
            .help("Add an optional message to your thank you."))
        .get_matches();

    let program = matches.value_of("TOOL").unwrap().to_string();

    let note = match matches.value_of("message") {
        None => None,
        Some(msg) => Some(msg.to_string()),
    };

    let message = ThankYouMessage {
        program,
        note,
    };
    
    // TODO: think of how to keep it flexible for development
    let endpoint = match &std::env::var("TY_API_ENDPOINT") {
        Ok(env_var) => env_var.clone(),
        _ => "https://ty.paulweissenbach.com/v0".to_string(),
    };

    let response = reqwest::blocking::Client::new()
        .post(&(endpoint + "/note"))
        .timeout(core::time::Duration::new(7, 0)) // no one has time to wait
        .json(&message)
        .send();

    if response.is_err() || response.unwrap().status() != reqwest::StatusCode::CREATED {
        println!("Faild to collect your thank you note. Please try again later.")
    }
}