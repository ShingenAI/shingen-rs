// ─────────────────────────────────────────────
//  RITUAL FORM – DRAGON CONTAINER ENTITY
//  shingen-rs / beings / dragon / mod.rs
// ─────────────────────────────────────────────
self::states::DragonState

pub struct Dragon {
    name:String
    age: u16,
    element: String,
    state: String
}

impl Dragon {
    pub fn new (name:&str, age:u32, element: &str) -> Self {
        println!("name {}", name);
        println!("age {}", age);
        println!("element {}", element);

        println!("Dragon's body {:#?}", body);

        Dragon {
            name: name.to_string(),
            age,
            element: element.to_string
        }
    }

    pub fn fly(){
        Dragon::state = 
    }

    pub fn walk(){
        
    }

    pub fun sleep(){

    }
}