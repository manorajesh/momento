use movement::Watch;

fn main() {
    use std::time::Instant;
    let start = Instant::now();
    let mut watch = Watch::new("01:34 AM", false);
watch += "01:23:45";
watch -= 1000000000;
println!("{}", watch);
    let duration = start.elapsed();
    println!("Time elapsed in main() is: {:?}", duration);
}