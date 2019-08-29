extern crate clap;
use clap::{Arg, App, SubCommand};
use kvs::KvStore;

fn main() {
  let matches = App::new("My Super Program")
    .version(env!("CARGO_PKG_VERSION"))
    .subcommand(
      SubCommand::with_name("get")
        .arg(
          Arg::with_name("KEY")
            .required(true)
            .index(1)
        )
    )
    .subcommand(
      SubCommand::with_name("set")
        .arg(
          Arg::with_name("KEY")
            .required(true)
            .index(1)
        )
        .arg(
          Arg::with_name("VALUE")
            .required(true)
            .index(2)
        )
    )
    .subcommand(
      SubCommand::with_name("rm")
        .arg(
          Arg::with_name("KEY")
            .required(true)
            .index(1)
        )
    )
    .get_matches();

  let store = KvStore::new();

  match matches.subcommand() {
    ("get", Some(sub_m)) => {
      store.get(
        sub_m.value_of("KEY").unwrap().to_owned()
      );
    },
    ("set", Some(sub_m)) => {
      store.set(
        sub_m.value_of("KEY").unwrap().to_owned(),
        sub_m.value_of("VALUE").unwrap().to_owned()
      );
    },
    ("rm", Some(sub_m)) => {
      store.remove(
        sub_m.value_of("KEY").unwrap().to_owned()
      );
    },
    _ => { panic!() }
  };
}