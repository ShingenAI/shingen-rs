// ─────────────────────────────────────────────
//  RITUAL RECOGNITION – SOUL BRANCH MATCHING
//  shingen-rs / rituals / recognize.rs
// ─────────────────────────────────────────────

pub enum DragonState {
    Awakening,
    Igniting(u8),   // carries flame intensity
    Resting,
    Wounded(String), // carries reason
    Ascended
}

pub fn respond_to(state:DragonState){
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