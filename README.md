# movement
![Build Status](https://github.com/manorajesh/movement/actions/workflows/rust.yml/badge.svg)
![Downloads](https://img.shields.io/crates/d/movement)
![Version](https://img.shields.io/crates/v/movement)
![License](https://img.shields.io/crates/l/movement)

Simple library and command to help out with time calculations (e.g. 1:23 PM + 3:30)

## Installation
```shell
cargo add movement
```

## Usage
```
fn adding_with_str() {
    let mut watch = Watch::new("2:15:01 A.M", true);
    watch += "3:14";
    println!("{}", watch);
    // outputs 05:29:01 AM
}

fn subtracting_with_secs() {
    let mut watch = Watch::new("13:34", true);
    watch += 4343;
    println!("{}", watch);
    // outputs 02:46:23 PM
}
```