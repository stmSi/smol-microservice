use std::env;
use std::net::SocketAddr;
use std::process::exit;
use std::sync::{Arc, Mutex};

use env_logger::Env;
use hyper::service::service_fn;
use hyper::Server;
use log::{info, error, trace};
use slab::Slab;
use hyper::rt::Future;

use hyper_microservice::microservice_handler;

use clap::{crate_authors, crate_version, crate_description, Arg, App, crate_name};


fn main() {
    
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("address")
            .short("a")
            .long("address")
            .value_name("ADDRESS")
            .help("Sets an address")
            .takes_value(true))
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true))
        .get_matches();


    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    info!("Rand Microservice - v0.1.0");
    let addr = matches.value_of("address")
        .map(|s| s.to_owned())
        .or(env::var("ADDRESS").ok())
        .unwrap_or_else(|| "127.0.0.1:6969".into())
        .parse()
        .expect("can't parse ADDRESS variable");

    info!("Trying to bind server to address: {}", addr);
    let builder = match Server::try_bind(&addr) {
        Ok(builder) => {
            info!("Binding Success: {}", &addr);
            builder
        },
        Err(err) => {
            error!("{}", &err);
            exit(-1);
        }
    };

    let user_db = Arc::new(Mutex::new(Slab::new()));

    trace!("Creating service handler...");
    let server = builder.serve(move || {
        let user_db = user_db.clone();
        service_fn(move |req| microservice_handler(req, &user_db))
    });
    let server = server.map_err(drop);
    hyper::rt::run(server);
}
