extern crate clap;

use clap::{App, Arg};

use kvs::Command;
use kvs::EngineStore;
use kvs::KvStore;
use kvs::Logger;
use kvs::Response;
use kvs::Result;
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::str;

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

    let address = matches.value_of("address").unwrap_or("127.0.0.1:4000");
    let engine = matches.value_of("engine").unwrap_or("kvs");

    info!(target: "address", "{:?}", address);
    info!(target: "engine", "{:?}", engine);

    let dir = std::env::current_dir().unwrap();

    let mut engine_store = EngineStore::new(dir)?;

    let last_engine = engine_store.get();

    if last_engine.is_empty() || last_engine == engine {
        engine_store.set(engine);
    } else {
        Err(format_err!("Engine does not match"))?
    }

    let listener = TcpListener::bind(address)?;

    let dir = std::env::current_dir().unwrap();
    let mut store = KvStore::open(dir).unwrap();

    for stream in listener.incoming() {
        let mut stream = stream?;
        let mut data = [0 as u8; 128];
        let size = stream.read(&mut data)?;
        let data_vec = data[..size].to_vec();
        let string = str::from_utf8(&data_vec)?;

        let result = get_result(string, &mut store);
        let response = Response::new(result);

        stream.write(serde_json::to_string(&response).unwrap().as_bytes())?;
    }

    Ok(())
}

fn get_result(string: &str, store: &mut KvStore) -> Result<Option<String>> {
    let result = match serde_json::from_str(string).unwrap() {
        Command::Set { key, value } => {
            store.set(key, value)?;

            None
        }
        Command::Get { key } => store.get(key)?,
        Command::Remove { key } => {
            store.remove(key)?;
            None
        }
    };

    Ok(result)
}
