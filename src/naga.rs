use evdev_rs::{Device, GrabMode, InputEvent, ReadStatus, BLOCKING, NORMAL};
use std::fs::{read_dir, File};

pub struct Naga {
    device: Device,
    // need to keep this file, otherwise file would be closed too early
    _file: File,
}

impl Naga {
    pub fn new() -> Result<Naga, String> {
        let paths = read_dir("/dev/input")
            .map_err(|e| format!("Problem reading input devices dir: {}", e))?;

        for path_result in paths {
            let path = match path_result {
                Ok(p) => p,
                Err(_) => {
                    continue;
                }
            };

            let file = match File::open(path.path()) {
                Ok(f) => f,
                Err(_) => {
                    continue;
                }
            };

            let mut device = match Device::new_from_fd(&file) {
                Ok(d) => d,
                Err(_) => {
                    continue;
                }
            };

            if device.name().unwrap_or("").eq("Razer Razer Naga 2014")
                && device.phys().unwrap_or("").ends_with("/input2")
            {
                device
                    .grab(GrabMode::Grab)
                    .map_err(|e| format!("Could not grab device: {}", e))?;
                return Ok(Naga {
                    device,
                    _file: file,
                });
            }
        }

        Err("No device found".to_string())
    }

    pub fn next_event(&self) -> Result<(ReadStatus, InputEvent), String> {
        match self.device.next_event(NORMAL | BLOCKING) {
            Ok(res) => Ok(res),
            Err(errno) => Err(format!("Problem reading event: {}", errno)),
        }
    }
}
