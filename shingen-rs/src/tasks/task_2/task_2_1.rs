use std::collection::HashMap;

pub fn execute_task(){
    println!("Task 2");

    let mut sat_scores: HashMap<String, i32> = HashMap::new();

    sat_scores.insert("Shin-1".to_string(), 72);
    sat_scores.insert(String::from("Shin-2"), 85);
    sat_scores.insert("Shin-3".into(), 90);
}