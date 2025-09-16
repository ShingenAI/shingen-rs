use std::collections::HashMap;

pub fn execute_task(query: &str){
    println!("Task 2");

    let mut sat_scores: HashMap<String, i32> = HashMap::new();

    sat_scores.insert("Shin-2".to_string(), 72);
    sat_scores.insert(String::from("Shin-3"), 85);
    sat_scores.insert("Shin-1".into(), 90);

    println!("Query: {}", query);
    if sat_scores.contains_key(query){
        println!("{} exists in the map", query);
    }else{
        println!("{} not found", query);
    }

    // Get value safety
    match sat_scores.get(query) {
        Some(score) => println!("{} score: {}", query, score),
        None => println!("{} has not match", query),
    }

    let mut keys: Vec<&String> = sat_scores.keys().collect();

    keys.sort();

    for key in keys {
        let val = sat_scores.get(key).unwrap();
        println!("{}-{}", key, val);
    }
}