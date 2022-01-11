# rz-fluid

rz-pipes, but with UNIX pipes. Replace a rizin command such as `pdg @ $(is~mag[1])` with `is~mag[1] | pdg @`.

## Example
```
cargo r -- --bin ../dump1090_rs/target/release/dump1090 --cmd "is~mag[1] | pdg @"
```

## Usage
```
rz-fluid

USAGE:
    rz-fluid --bin <BIN> --cmd <CMD>

OPTIONS:
        --bin <BIN>
        --cmd <CMD>
    -h, --help         Print help information
```
