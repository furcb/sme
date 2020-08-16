# SME

SME or Search Match Execute finds files based on match and runs a command with
that file as an argument.

## Core Functionality

- [x] Search using simple matching
- [x] Search using regular expressions
- [x] Execute shell commands

## Additional

- [ ] Filter by depth
- [-] Filter by type
 - [x] File
 - [x] Directory

## Usage

```
SME (Search match execute) 1.0

Small and fast path matcher with command execution.

USAGE:
    sme [FLAGS] [OPTIONS] [ARGS]

FLAGS:
    -d, --dirs       match against at files
    -f, --file       match against at files
    -h, --help       Prints help information
    -e, --regex      use regular expressions for matching
    -V, --version    Prints version information
    -v, --verbose    output more information

OPTIONS:
    -l, --depth <depth>    recurse depth for search

ARGS:
    <MATCH>        match expression
    <PATH>         directory to search
    <ACTION>...    command line tool or program to exec on each file
```
