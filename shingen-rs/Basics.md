Living Recursion:

# Rust Toolchain
- rustc (the compiler)
- cargo (the ritual orchestrator)
- rust-std (the standard library)
- and keeps everything updated

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo new shingen-rs --vcs none
cd shingen-rs

# String and &string
From reflection to scroll?
🛠️ The ritual (.to_string()) is the act of giving the reflection mass.
This is OCR from the mirror, as you beautifully said.

let reflection: &str = "Shinryū";
let new_scroll: String = reflection.to_string();

From scroll to reflection?
🪞 The mirror does not steal.
It simply borrows the light.

let scroll: String = String::from("Shinryū");
let reflection: &str = &scroll;

# Ownership and borrow
// 🔱 FULL OWNERSHIP
fn consume(dragon: DragonForm) { ... }
// 📜 “The altar is moved to a new shrine.
//     Anyone who wants to access it now
//     must come to the new shrine.
//     The original shrine no longer has it.”


// 🪞 IMMUTABLE BORROW
fn read(dragon: &DragonForm) { ... }
// 🕊️ “Allows guests to enter the shrine
//     and gaze upon the altar.
//     They may reflect, but never alter.
//     The altar is safe and unchanged.”

// ⚔️ MUTABLE BORROW
fn change(dragon: &mut DragonForm) { ... }
// 🔧 “Grants exclusive access to reshape the altar
//     inside the original shrine.
//     Only one soul may touch it at a time.
//     Once reshaping ends, the shrine reopens.”

let s: String = String::from("Shinryū");
let r: &str = &s; // borrowing the string

# Vectors
pub fn execute_task(name_to_search:&String){
    println!("Name to search: {}", name_to_search);
    
    let mut satlite_names = vec!["Shin chan".to_string(), "Magokoro".to_string(), "Ghost".to_string()];
    
    satlite_names.push("Chushinden".to_string());

    satlite_names.remove(2);

    if satlite_names.contains(name_to_search) {
        println!("The name {} exists.", name_to_search);
    }

    for (_index, _name) in satlite_names.iter().enumerate() {
        println!("{} - {}", index, name);
    }
}


# HashMap
use std::collections::HashMap;
use super::sat_struct::SatData;

pub fn execute_task(query: &str){
    println!("Task 2");

    let mut sat_scores: HashMap<String, SatData> = HashMap::new();

    sat_scores.insert("Shin-2".to_string(), SatData {
        name:"Shin-2".to_string(),
        temperature: 72.into(),
        voltage: 110.into()
    });
    sat_scores.insert(String::from("Shin-3"), SatData {
        name:"Shin-3".to_string(),
        temperature: 85.into(),
        voltage: 109.into()
    });
    sat_scores.insert("Shin-1".into(),  SatData {
                                            name:"Shin-1".to_string(),
                                            temperature: 90.into(),
                                            voltage: 105.into()
                                        }   
    );

    println!("Query: {}", query);
    if sat_scores.contains_key(query){
        println!("{} exists in the map", query);
    }else{
        println!("{} not found", query);
    }

    // Get value safety
    match sat_scores.get(query) {
        Some(sat_data) => {
            println!("Sat name: {}",      sat_data.name);
            println!("\ttemperature: {}", sat_data.temperature);
            println!("\tvoltage: {}",     sat_data.voltage);
        },
        None => println!("{} has not match", query)
    }

    let mut keys: Vec<&String> = sat_scores.keys().collect();

    keys.sort();

    for key in keys {
        if let Some(val) = sat_scores.get(key) {
            println!("{}-{:#?}", key, val);
        }
    }
}

# Formatter
 #[derive(Debug)]
pub struct SatData {
    pub name:String,
    pub temperature: f64,
    pub voltage: f64
}

{:#?}