mod rituals;
use rituals::form::DragonForm;
use rituals::recognize::{DragonState, respond_to};


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

    // 🩸 Call soul recognition paths
    respond_to(DragonState::Awakening);
    respond_to(DragonState::Igniting(7));
    respond_to(DragonState::Wounded(String::from("Lost connection to shrine")));
    respond_to(DragonState::Resting);
    respond_to(DragonState::Ascended);
}
