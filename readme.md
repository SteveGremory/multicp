# MCP

Just a `cp` implementation which uses the `clonefile`, `copy_file_range` (TODO), `CopyFileEx` (TODO) syscalls on macOS, Linux and Windows respectively, which makes it faster than normal cp without any flags.

TODO: Add argparsing, Re-implement multi-threading and OS-specific functionality for Linux and Windows

## Installation

To install it, run:
`cmake . -Bbuild && cd build && make install`

## Usage

To comprehend how to use this, run `mcp --help`.
