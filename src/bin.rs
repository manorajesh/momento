use movement::Watch;

fn main() {
    let mut watch = Watch::new("13:34", true);
    watch += 4343;
    println!("{}", watch);
    // outputs 05:29:01 AM
}
