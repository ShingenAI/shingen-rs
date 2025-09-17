use std::collections::HashMap;
use self::sat_struct::Sat_data;

pub fn execute_task(query: &str){
    println!("Task 2");

    let mut sat_scores: HashMap<String, Sat_data> = HashMap::new();

    sat_scores.insert("Shin-2".to_string(), Sat_data {
        name:"Shin-2".to_string(),
        temperature: 72,
        voltage: 110
    });
    sat_scores.insert(String::from("Shin-3"), Sat_data {
        name:"Shin-3".to_string(),
        temperature: 85,
        voltage: 109
    });
    sat_scores.insert("Shin-1".into(),  Sat_data {
                                            name:"Shin-1".to_string(),
                                            temperature: 90,
                                            voltage: 110
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
            println!("{}-{}", key, val);
        }
    }
}