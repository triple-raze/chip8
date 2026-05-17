use std::fs::File;
use std::io::Read;

pub fn read_rom(path: &String) -> [u8; 0x800] {
    let mut file = File::open(path).unwrap_or_else(|err| {
        eprintln!("Failed to read ROM: {}", err);
        std::process::exit(1)
    });

    let mut buffer = [0; 0x800];
    file.read(&mut buffer).unwrap();

    buffer
}
