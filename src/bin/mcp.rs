use std::path::PathBuf;

use multicp::copy;
use std::io;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "A faster cp")]

struct Args {
    /// Input file/folder
    from: PathBuf,

    /// Output file/folder
    to: PathBuf,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    copy(args.from, args.to)?;

    Ok(())
}
