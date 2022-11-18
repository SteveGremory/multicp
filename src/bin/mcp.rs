// Made by SteveGremory: Licenced under the MIT license.
use clap::Parser;
use multicp::copy;
use std::io;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about = "A faster cp")]

struct Args {
    /// Input file/folder
    from: PathBuf,

    /// Output file/folder
    to: PathBuf,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let args = Args::parse();

    copy(args.from, args.to).await?;

    Ok(())
}
