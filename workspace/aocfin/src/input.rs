use eyre::Result;
use std::{
    fs::File,
    io::{Read, Write},
};
use ureq::get;

static BYTES_LIMIT: u64 = 1_000_000;

pub fn download(token: &str, year: &u16, day: &u8) -> Result<Vec<u8>> {
    let response = get(&format!("https://adventofcode.com/{year}/day/{day}/input")).set("Cookie", &format!("session={token}")).call()?;

    let mut bytes = Vec::new();
    response.into_reader().take(BYTES_LIMIT).read_to_end(&mut bytes)?;

    if bytes.len() < BYTES_LIMIT as usize {
        Ok(bytes)
    } else {
        Err(eyre::eyre!(
        "input is at least {BYTES_LIMIT} bytes, which is probably an error"
    ))
    }
}

pub fn save(outfile: &str, data: &[u8]) -> Result<()> {
    let mut file = File::options().create_new(true).write(true).open(outfile)?;
    file.write_all(data)?;

    Ok(())
}
