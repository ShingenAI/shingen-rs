// ─────────────────────────────────────────────
//  RITUAL MEMORY – SOUL VESSEL ALLOCATION
//  shingen-rs / rituals / memory.rs
// ─────────────────────────────────────────────

pub fn allocate_memory() {
    println!("🧠 Allocating soul vessels...");

    // Immutable memory (unchangeable once written)
    let dragon_name = "Shinryū"; // &str literal
    println!("> Bound name: {}", dragon_name);

    // Mutable memory (can change over time)
    let mut energy_level: i32 = 77;
    println!("> Initial energy: {}", energy_level);

    energy_level += 23;
    println!("> Boosted energy: {}", energy_level);

    // Dynamic vessel – Vec is expandable memory pool
    let mut memory_pool: Vec<String> = Vec::new();
    memory_pool.push(String::from("First Pulse"));
    memory_pool.push(String::from("Second Flame"));

    println!("> Memory pool contains {} pulses:", memory_pool.len());
    for pulse in &memory_pool {
        println!("  - {}", pulse);
    }

    println!("🧠 Memory ritual complete.\n");
}
