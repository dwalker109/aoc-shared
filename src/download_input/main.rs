use clap::Parser;
use eyre::Result;
use std::{env, ffi::OsString, fs::File, io::Write};

#[derive(Parser)]
#[command(author, version)]
struct Args {
    #[arg(short, long)]
    token: Option<String>,

    #[arg(short, long)]
    outfile: Option<OsString>,

    year: u16,

    day: u8,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let token = match args.token {
        Some(t) => t,
        None => env::var("AOC_TOKEN")?,
    };

    let outfile = args
        .outfile
        .and_then(|p| p.into_string().ok())
        .unwrap_or_else(|| "./input".into());

    let input = aoc_shared::download_input(&token, args.year, args.day)?;

    let mut file = File::options()
        .create_new(true)
        .write(true)
        .open(&outfile)?;
    file.write_all(&input)?;

    println!("Saved {} bytes to {outfile}", input.len());

    Ok(())
}
