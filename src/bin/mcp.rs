use std::path::Path;

use multicp::copy;
use tokio::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let from = Path::new("/Users/steve/Downloads");
    let to = Path::new("idk");

    copy(from.to_path_buf(), to.to_path_buf()).await?;

    Ok(())
}
