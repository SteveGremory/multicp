# MultiCP
Just a multithreaded version of `cp` which uses the `fcopyfile`, `copy_file_range`, `CopyFileEx` syscalls on macOS, Linux and Windows respectively (abstracted by the Rust Standard lib's in the form of `fs::copy`), which makes it faster than normal cp.

If a directory has been copied over by MultiCP before and some files inside the folder have not been modified, then they will be skipped if there are no changes among the new and the old file upon a re-copy. 

Basically, if monke copy with MultiCP, monke no modify file, monke faster transfer speed because monke no copy unchanged files.

## Installation
To install it, run:
`cargo install --path .`

To run it, run `mcp --help`.