extern crate clap;
extern crate failure;
extern crate failure_derive;

use clap::{App, Arg, SubCommand};
use kvs::KvStore;
use kvs::Result;

fn main() -> Result<()> {
    let matches = App::new("My Super Program")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .subcommand(SubCommand::with_name("get").arg(Arg::with_name("KEY").required(true).index(1)))
        .subcommand(
            SubCommand::with_name("set")
                .arg(Arg::with_name("KEY").required(true).index(1))
                .arg(Arg::with_name("VALUE").required(true).index(2)),
        )
        .subcommand(SubCommand::with_name("rm").arg(Arg::with_name("KEY").required(true).index(1)))
        .get_matches();

    let dir = std::env::current_dir().unwrap();
    let mut store = KvStore::open(dir).unwrap();

    match matches.subcommand() {
        ("get", Some(sub_m)) => {
            match store.get(sub_m.value_of("KEY").unwrap().to_owned())? {
                Some(value) => println!("{}", value),
                None => println!("Key not found"),
            }

            Ok(())
        }
        ("set", Some(sub_m)) => {
            store.set(
                sub_m.value_of("KEY").unwrap().to_owned(),
                sub_m.value_of("VALUE").unwrap().to_owned(),
            )?;

            Ok(())
        }
        ("rm", Some(sub_m)) => match store.remove(sub_m.value_of("KEY").unwrap().to_owned()) {
            Err(error) => {
                println!("{}", error.as_fail());
                Err(error)
            }
            Ok(_) => Ok(()),
        },
        _ => panic!(),
    }
}
