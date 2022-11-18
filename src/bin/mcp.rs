use std::path::PathBuf;

use multicp::copy;
use std::io;

fn main() -> io::Result<()> {
    let from = PathBuf::from("/Users/steve/Downloads");
    let to = PathBuf::from("idk");

    copy(from, to)?;

    Ok(())
}
