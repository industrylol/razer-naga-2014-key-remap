use std::time::Duration;
use std::{env, thread};

mod event_mapper;
mod input_device;
mod key_map;
mod naga;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() -> Result<(), String> {
    println!("razer-naga-2014-key-remap v{}", VERSION);

    let args: Vec<String> = env::args().collect();

    let key_mapper = match args.len() {
        2 => key_map::KeyMapper::read_from_file(args[1].as_str())?,
        1 => key_map::KeyMapper::default(),
        _ => {
            return Err("Too many arguments".to_string());
        }
    };

    let mut device = input_device::create()?;

    loop {
        let naga = naga::Naga::new();

        match naga {
            Ok(dev) => {
                println!("Attached to naga");
                let res = event_mapper::map_events(key_mapper, dev, &mut device);
                match res.err() {
                    Some(e) => eprintln!("Error mapping events: {}", e),
                    None => eprintln!("Map events returned Ok which was not expected"),
                }
            }
            Err(err) => eprintln!("Error looking for naga: {}", err),
        }
        thread::sleep(Duration::from_secs(1))
    }
}
