use std::fs;
use std::io::Read;
use std::path::PathBuf;

pub const BASE_ADDRESS: u16 = 0x200;

pub fn load(path: &PathBuf) -> std::io::Result<Vec<u8>> {
    let mut file = fs::File::open(path)?;
    let mut buffer = vec![];
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
