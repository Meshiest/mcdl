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

### Windows

To create shortcuts that will automatically download the latest snapshot:

1. Right click on `mcdl.exe` and click "Create Shortcut"
2. Right click on the newly created shortcut and click "Properties"
3. In the "Target" text area, add "-s" to the end

You can replace the "-s" in step 3 with any of the other flags from the examples above or the usage below

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

## Why?

I thought it would be fun to write in Rust. If you wanted a single line of bash to do it, use one of these (requires `jq`, `curl`, and `wget`):

- Download latest available (snapshot or release) minecraft server.jar

  ```sh
  curl -s $(curl -s "https://launchermeta.mojang.com/mc/game/version_manifest.json" | jq -r ".versions[0].url") | jq -r ".downloads.server.url" | xargs wget
  ```

- Download latest release minecraft server.jar

  ```sh
  curl -s $(curl -s "https://launchermeta.mojang.com/mc/game/version_manifest.json" | jq -r ".latest.release as \$v | .versions[] | select(.id == \$v) | .url") | jq -r ".downloads.server.url" | xargs wget
  ```

- Download latest snapshot minecraft server.jar

  ```sh
  curl -s $(curl -s "https://launchermeta.mojang.com/mc/game/version_manifest.json" | jq -r ".latest.snapshot as \$v | .versions[] | select(.id == \$v) | .url") | jq -r ".downloads.server.url" | xargs wget
  ```

## Example Scripts

### (Windows) Automatically install latest snapshot, then start minecraft server

```bat
@echo OFF
@REM if a version file exists, check if it's the latest, otherwise download the server
if exist version.txt (
  goto checkVersion
) else (
  goto download
)

:checkVersion
@REM compare latest version with the one that is installed
mcdl.exe -sp > temp.version.txt
fc /b temp.version.txt version.txt > nul
if errorlevel 1 (
  @REM on different version - install the game
  mcdl.exe -s
  move /y temp.version.txt version.txt > nul
) else (
  @REM on same version delete the temp file
  del temp.version.txt > nul
)
goto server

@REM download the game and update the version
:download
mcdl.exe -s
mcdl.exe -sp > version.txt

@REM start the server
:server
java -Xmx4G -Xms2G -jar server.jar nogui

timeout /t 5
goto server
pause
```
