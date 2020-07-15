extern crate clap;
#[macro_use]
extern crate failure;
extern crate failure_derive;

use clap::{App, Arg, SubCommand};
use kvs::Command;
use kvs::Response;
use kvs::Result;
use std::io::prelude::*;
use std::io::Read;
use std::net::TcpStream;

fn main() -> Result<()> {
    let matches = App::new("KVS Client")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand(
            SubCommand::with_name("get")
                .arg(Arg::with_name("KEY").required(true).index(1))
                .arg(Arg::with_name("address").long("addr").takes_value(true)),
        )
        .subcommand(
            SubCommand::with_name("set")
                .arg(Arg::with_name("KEY").required(true).index(1))
                .arg(Arg::with_name("VALUE").required(true).index(2))
                .arg(Arg::with_name("address").long("addr").takes_value(true)),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .arg(Arg::with_name("KEY").required(true).index(1))
                .arg(Arg::with_name("address").long("addr").takes_value(true)),
        )
        .get_matches();

    let command = match matches.subcommand() {
        ("get", Some(sub_m)) => Command::Get {
            key: sub_m.value_of("KEY").unwrap().to_owned(),
        },
        ("set", Some(sub_m)) => Command::Set {
            key: sub_m.value_of("KEY").unwrap().to_owned(),
            value: sub_m.value_of("VALUE").unwrap().to_owned(),
        },
        ("rm", Some(sub_m)) => Command::Remove {
            key: sub_m.value_of("KEY").unwrap().to_owned(),
        },
        _ => panic!(),
    };

    let address = matches
        .subcommand()
        .1
        .unwrap()
        .value_of("address")
        .unwrap_or("127.0.0.1:4000");
    let mut stream = TcpStream::connect(address)?;

    stream.write(serde_json::to_string(&command).unwrap().as_bytes())?;

    let mut buffer = String::new();
    stream.read_to_string(&mut buffer)?;

    let response: Response = serde_json::from_str(&buffer)?;

    if response.is_error() {
        Err(format_err!("{}", response.error.unwrap()))
    } else {
        match matches.subcommand().0 {
            "get" => match response.value {
                Some(value) => println!("{}", value),
                None => println!("Key not found"),
            },
            _ => {}
        }

        Ok(())
    }
}
