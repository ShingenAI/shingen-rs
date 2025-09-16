pub fn execute_task(name_to_search:&String){
    //println!("Name to search: {}", name_to_search);
    
    let mut satlite_names = vec!["Shin chan".to_string(), "Magokoro".to_string(), "Ghost".to_string()];
    
    satlite_names.push("Chushinden".to_string());

    satlite_names.remove(2);

    if satlite_names.contains(name_to_search) {
        //println!("The name {} exists.", name_to_search);
    }


    for (_index, _name) in satlite_names.iter().enumerate() {
        //println!("{} - {}", index, name);
    }
}