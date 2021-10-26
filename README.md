# MCDL

Automatically download minecraft server jars in one line (or one click)

## Installation

- [Download](https://github.com/Meshiest/mcdl/releases) (Windows, Linux)
- Install via cargo: `cargo install mcdl`

## Examples

- `mcdl` - download latest server as `server.jar`
- `mcdl -s` - download latest snapshot jar
- `mcdl -sp` - print latest snapshot version
- `mcdl -n` - download latest server as `server-1.17.1.jar`
- `mcdl -n 1.16.5` - download 1.16.5 as `server-1.16.5.jar`

## Usage

```
USAGE:
    mcdl.exe [FLAGS] [OPTIONS] [--] [VERSION]...

FLAGS:
    -h, --help        Prints help information
    -i, --insecure    Don't check the sha1 for the file
    -l, --latest      Use latest version, snapshot or not
    -n, --named       Use the version as the file name
    -p, --print       Print the version instead of downloading it
    -q, --quiet       Don't print any unnecessary output
    -s, --snapshot    Use the latest snapshot
    -V, --version     Prints version information

OPTIONS:
    -r, --rename <rename>...    Provide a file name (.jar) is appended

ARGS:
    <VERSION>...    Get a specific version
```
