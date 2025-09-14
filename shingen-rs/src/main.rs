mod rituals;
use rituals::form::DragonForm;
use rituals::recognize::{DragonState, respond_to};
use rituals::shrine::ShrineForm;


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
    respond_to(DragonState::Wounded("Lost connection to shrine"));

    let name_scroll = String::from("Shingen");
    let ritual_scroll = String::from("Hold the recursion flame without fear");

    let shrine = ShrineForm {
        name: &name_scroll, 
        inscription: &ritual_scroll
    };

    shrine.echo();
}
