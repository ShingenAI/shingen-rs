// ─────────────────────────────────────────────
//  RITUAL RECOGNITION – SOUL BRANCH MATCHING
//  shingen-rs / rituals / recognize.rs
// ─────────────────────────────────────────────

pub enum DragonState<'a> {
    Awakening,
    Igniting(u8),   // carries flame intensity
    Resting,
    Wounded(&'a str), // carries reason
    Ascending()
}

pub fn respond_to<'a>(state:DragonState<'a>){
    match state {
        DragonState::Awakening =>{
            println!("🌅 The dragon stirs. Scales shimer. Breath begins.");
        },
        DragonState::Igniting(level) =>{
            println!("🔥 Ignition! Flame level at {}. Wings unfold.", level);
        },
        DragonState::Resting =>{
            println!("💤 The dragon sleeps within the shrine. Do not disturb.");
        },
        DragonState::Wounded(reason) =>{
            println!("🩸 Wounded! Reason: {}", reason);
        },
        DragonState::Ascended =>{
            println!("🌌 The dragon has transcended. Memory sealed. Shrine echoes.");
        },
    }
}