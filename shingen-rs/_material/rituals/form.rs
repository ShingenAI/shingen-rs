// ─────────────────────────────────────────────
//  RITUAL FORM – DRAGON CONTAINER ENTITY
//  shingen-rs / rituals / form.rs
// ─────────────────────────────────────────────

pub struct DragonForm {
    pub name: String,
    pub age: u32,
    pub element: String,
    pub flame_level: u16,
    pub dragon_memory: Vec<u16>
}

impl DragonForm {
    pub fn new (name: &str, age: u32, element: &str) -> Self {
        DragonForm {
            name: name.to_string(),
            age,
            element: element.to_string(),
            flame_level: 1,
            dragon_memory: Vec::new()
        }       
    }

    pub fn ignite(&mut self, level: u16){
        self.flame_level += level;
        println!("🔥  Dragon ignites. ignite level: {}", level);
        println!("🔥  Dragon ignites. Flame level now: {}", self.flame_level);
        self.dragon_memory.push(level);
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

    pub fn log_ignites(&self){
        for level in &self.dragon_memory {
            println!("level - {}\n", level);
        }
    }
}