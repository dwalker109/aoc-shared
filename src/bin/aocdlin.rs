use eyre::Result;
use pico_args::Arguments;
use std::{env, fs::File, io::Write};

static USAGE: &str =
    "Usage: aocdlin [-T/--token <AOC_TOKEN>] [-O/--outfile <OUTFILE>] <YEAR> <DAY>";

fn main() -> Result<()> {
    let mut args = Arguments::from_env();

    let token = args
        .opt_value_from_str::<_, String>(["-T", "--token"])?
        .or_else(|| env::var("AOC_TOKEN").ok())
        .unwrap_or_else(|| panic!("{USAGE} (help: provide a token via arg or AOC_TOKEN env var)"));

    let outfile = args
        .opt_value_from_str::<_, String>(["-O", "--outfile"])?
        .unwrap_or_else(|| "./input".into());

    let year = args.free_from_str::<u16>().expect(USAGE);
    let day = args.free_from_str::<u8>().expect(USAGE);

    let input = aoc_shared::download_input(&token, &year, &day)?;
    let mut file = File::options()
        .create_new(true)
        .write(true)
        .open(&outfile)?;
    file.write_all(&input)?;

    println!("Saved {} bytes to {}", input.len(), &outfile);

    Ok(())
}
