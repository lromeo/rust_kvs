extern crate clap;

use clap::{App, Arg};

use kvs::EngineStore;
use kvs::Logger;
use kvs::Result;

#[macro_use]
extern crate failure;

use log::{info, LevelFilter};

static LOGGER: Logger = Logger;

fn main() -> Result<()> {
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
        .expect("unable to set logger");

    info!("version: {:?}", env!("CARGO_PKG_VERSION"));

    let matches = App::new("KVS Server")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::with_name("address").long("addr").takes_value(true))
        .arg(Arg::with_name("engine").long("engine").takes_value(true))
        .get_matches();

    let address = matches.value_of("address").unwrap();
    let engine = matches.value_of("engine").unwrap();

    info!(target: "address", "{:?}", address);
    info!(target: "engine", "{:?}", engine);

    let dir = std::env::current_dir().unwrap();

    let mut engine_store = EngineStore::new(dir)?;

    let last_engine = engine_store.get();

    if last_engine.is_empty() || last_engine == engine {
        engine_store.set(engine);
        Ok(())
    } else {
        Err(format_err!("Engine does not match"))
    }
}
