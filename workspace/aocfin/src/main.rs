use eyre::Result;
use pico_args::Arguments;
use std::env;
use aocfin::input;

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

    let data = input::download(&token, &year, &day)?;
    input::save(&outfile, &data)?;

    println!("Saved {} bytes to {}", data.len(), &outfile);

    Ok(())
}
