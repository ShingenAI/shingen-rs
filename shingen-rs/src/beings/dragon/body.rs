// ─────────────────────────────────────────────
//  RITUAL FORM – DRAGON CONTAINER ENTITY
//  shingen-rs / beings / dragon / body.rs
// ─────────────────────────────────────────────
pub structure DragonBody {
    pub age: u32,
    pub element: String,
}

impl DragonBody {
    pub fn new (age:u32, element: &str) -> Self {
        DragonForm {
            age,
            element: element.to_string()
        }
    }

    pub fn fly(&mut self){
        println!("Dragon flying");
    }

    pub fn walk(&mut self, level:u32){
         println!("Dragon walking");
    }

    pub fn stand(&mut self, level:u32){
         println!("Dragon stand");
    }
}