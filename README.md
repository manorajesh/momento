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
Initialize a watch with the `start time` and `meridiem` option as the arguments. The start time 
can be in either 12h or 24h format. The meridiem option is a `bool` that represents whether 
the watch is in 12h or 24h format (true for 12h). It can be changed with the method `change_meridiem()`. The default display format is `HH:MM:SS` 
and +/- days.
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

### Examples
See [the tests](https://github.com/manorajesh/movement/blob/a829ca76bc177d55f4ea0f34a6ac82302a7eb445/src/lib.rs#L234)
for more examples