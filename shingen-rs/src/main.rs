mod beings;
use beings::dragon::Dragon;

fn main() {
    let mut _dragon = Dragon::new("Shinryū", 9999, "Soul Flame");
    println!("Dragon {:#?}", _dragon);
}