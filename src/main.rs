extern crate clap;
extern crate trow;

use clap::{Arg, ArgMatches};
use trow::{NetAddr, TrowBuilder};

const PROGRAM_NAME: &str = "Trow";
const PROGRAM_DESC: &str = "\nThe Cluster Registry";

/*
  Parses command line arguments and returns ArgMatches object.

  Will cause the program to exit if error or on help/version argument.
*/
fn parse_args<'a>() -> ArgMatches<'a> {
    clap::App::new(PROGRAM_NAME)
        .version("0.1")
        .author("From Container Solutions")
        .about(PROGRAM_DESC)
        .arg(
            Arg::with_name("host")
                .long("host")
                .value_name("host")
                .help("Sets the name of the host or interface to start Trow on. Defaults to 0.0.0.0")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .long("port")
                .value_name("port")
                .help("The port that trow will listen on. Defaults to 8443.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("no-tls")
                .long("no-tls")
                .help("Turns off TLS. Should only be used in development and debugging. If used in production, make sure you understand the risks.")
        )
        .arg(
            Arg::with_name("cert")
                .short("c")
                .long("cert")
                .value_name("cert")
                .help("Path to TLS certificate. Defaults to ./certs/ca.crt.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("key")
                .short("k")
                .long("key")
                .value_name("key")
                .help("Path to TLS private key. Defaults to ./certs/domain.key.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("data-dir")
                .short("d")
                .long("data-dir")
                .value_name("data_dir")
                .help("Directory to store images and metadata in.")
                .takes_value(true),
        )
        .get_matches()
}

fn main() {
    let matches = parse_args();

    let host = matches.value_of("host").unwrap_or("0.0.0.0");
    let port: u16 = matches
        .value_of("port")
        .map_or(8443, |x| x.parse().unwrap());
    let cert_path = matches.value_of("cert").unwrap_or("./certs/ca.crt");
    let key_path = matches.value_of("key").unwrap_or("./certs/domain.key");
    let data_path = matches.value_of("key").unwrap_or("./data");
    let no_tls = matches.is_present("no-tls");

    let addr = NetAddr {
        host: host.to_string(),
        port: port,
    };
    let grpc_listen = NetAddr {
        host: "127.0.0.1".to_owned(),
        port: 51000,
    };
    let grpc_boot = NetAddr {
        host: "127.0.0.1".to_owned(),
        port: 3117,
    };
    let mut builder = TrowBuilder::new(data_path.to_string(), addr, grpc_listen, grpc_boot);
    if !no_tls {
        builder.with_tls(cert_path.to_string(), key_path.to_string());
    }
    builder.start().unwrap_or_else(|e| {
        eprintln!("Error launching Trow {}", e);
        std::process::exit(1);
    });
}
