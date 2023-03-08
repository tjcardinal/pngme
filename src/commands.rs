use crate::{chunk::Chunk, png::Png, Result};
use std::{io::Read, path::PathBuf};

pub fn encode(
    filename: PathBuf,
    chunk_type: String,
    msg: String,
    output: Option<PathBuf>,
) -> Result<()> {
    let mut file = std::fs::File::open(filename.clone())?;
    let mut bytes = vec![];
    file.read_to_end(&mut bytes)?;

    let mut png = Png::try_from(&bytes[..])?;
    let chunk_type: [u8; 4] = chunk_type.as_bytes().try_into()?;
    let chunk = Chunk::new(chunk_type.try_into()?, msg.into_bytes());
    png.append_chunk(chunk);

    let output_file = match output {
        Some(o) => o,
        None => filename,
    };

    std::fs::write(output_file, png.as_bytes())?;
    Ok(())
}

pub fn decode(filename: PathBuf, chunk_type: String) -> Result<()> {
    let mut file = std::fs::File::open(filename)?;
    let mut bytes = vec![];
    file.read_to_end(&mut bytes)?;

    let png = Png::try_from(&bytes[..])?;
    match png.chunk_by_type(&chunk_type) {
        Some(c) => {
            println!("{}", c.data_as_string()?);
            Ok(())
        }
        None => Err("No chunk found".into()),
    }
}

pub fn remove(filename: PathBuf, chunk_type: String) -> Result<()> {
    let mut file = std::fs::File::open(filename.clone())?;
    let mut bytes = vec![];
    file.read_to_end(&mut bytes)?;

    let mut png = Png::try_from(&bytes[..])?;
    png.remove_chunk(&chunk_type)?;
    std::fs::write(filename, png.as_bytes())?;
    Ok(())
}

pub fn print(filename: PathBuf) -> Result<()> {
    let mut file = std::fs::File::open(filename)?;
    let mut bytes = vec![];
    file.read_to_end(&mut bytes)?;

    let png = Png::try_from(&bytes[..])?;
    println!("{}", png);

    Ok(())
}
