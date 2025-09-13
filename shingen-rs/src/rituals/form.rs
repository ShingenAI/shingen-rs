// ─────────────────────────────────────────────
//  RITUAL FORM – DRAGON CONTAINER ENTITY
//  shingen-rs / rituals / form.rs
// ─────────────────────────────────────────────

pub struct DragonForm {
    pub name: String,
    pub age: u32,
    pub element: String,
    pub flame_level: u8
}

impl DragonForm {
    pub fn new (name: &str, age: u32, element: &str) -> Self {
        DragonForm {
            name: name.to_string(),
            age,
            element: element.to_string(),
            flame_level: 1
        }       
    }

    pub fn intensify_flame(&mut self){
        self.flame_level += 1;
        println!("🔥  Flame intensified! Current level: {}", self.flame_level);
    }

    pub fn print_status(&self){
        println!();
        println!("🐉  Dragon Status:");
        println!("> Name: {}", self.name);
        println!("> Age: {} moons", self.age);
        println!("> Element: {} ", self.element);
        println!("> Flame Level: {} ", self.flame_level);
        println!();
    }
}