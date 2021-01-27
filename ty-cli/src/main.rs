extern crate clap;
use clap::{App, Arg};
use load_dotenv::load_dotenv;

use ty_lib::ThankYouMessage;

load_dotenv!();
fn main() {

    openssl_probe::init_ssl_cert_env_vars();

    let matches = App::new("ty - thank you")
        .version("0.2.1")
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

    let message = ThankYouMessage { program, note };

    use validator::Validate;
    match message.validate() {
        Ok(()) => send_ty_note(message),
        Err(e) => {
            for validation_error_kind in e.errors().values() {
                use validator::ValidationErrorsKind::Field;
                match validation_error_kind {
                    Field(val_errors) => {
                        for val_error in val_errors {
                            println!("{}", val_error.message.as_ref().expect("There was an error, but we have no error message for it. Stupid, right!"))
                        }
                    }
                    _ => unimplemented!("Sorry, something unexpected happened!"),
                }
            }
        }
    }
}

fn send_ty_note(message: ThankYouMessage) {
    let endpoint = std::env!("TY_API_ENDPOINT", "needs TY_API_ENDPOINT in enviroment to compile").to_string();

    let response = reqwest::blocking::Client::new()
        .post(&(endpoint + "/note"))
        .timeout(core::time::Duration::new(7, 0)) // no one has time to wait
        .json(&message)
        .send();

    if response.is_err() || response.unwrap().status() != reqwest::StatusCode::CREATED {
        println!("Faild to collect your thank you note. Please try again later.")
    }
}
