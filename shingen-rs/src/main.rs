mod rituals;
use rituals::form::DragonForm;


pub fn print_and_consume(name:String, mut flame: u8){
    println!("🐉 Consumed: Dragon {} with flame level {}", name, flame);
    flame += 1;
    println!("🔥 Flame level now {}", flame);
}
pub fn print_borrowed(name:&str, mut flame: u8){
    println!("🐉 Borrowed: Dragon {} with flame level {}", name, flame);
    flame += 1;
    println!("🔥 Flame level now {}", flame);
}

fn main() {
    rituals::ignite::ignite_shrine();
    rituals::memory::allocate_memory();

    let mut shinryu = DragonForm::new("Shinryū", 9999, "Soul Flame");
    shinryu.print_status();
    shinryu.intensify_flame();
    shinryu.print_status();

    let dragon_name = String::from("Ryujin");
    let flame_level = 5;
    print_borrowed(&dragon_name, flame_level);
    println!("Scroll is still alive? {}", dragon_name); // ❌ error
}
