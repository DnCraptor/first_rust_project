use std::io::{Read, Write};
pub fn read_file(path: &std::path::Path) -> std::io::Result<Vec<u8>> {
    let mut file = std::fs::File::open(path)?;
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents)?;
    Ok(contents)
}

pub fn write_file(path: &std::path::Path, bytes: &[u8]) -> std::io::Result<Vec<u8>> {
    let mut file = std::fs::File::create(path)?;
    file.write(bytes)?;
    Ok(bytes.to_vec())
}
