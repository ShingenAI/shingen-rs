// ─────────────────────────────────────────────
//  RITUAL SHRINE – BORROWED INSCRIPTION FORM
//  shingen-rs / rituals / shrine.rs
// ─────────────────────────────────────────────

pub struct ShrineForm<'a> {
    pub name: &'a str, // Mirror: borrowed scroll name
    pub inscription: &'a str // Mirror: borrowed scroll name
}

impl<'a> ShrineForm<'a> {
    pub fn echo(&self){
        println!("ShrineForm echo:");
        println!("> Name Scroll: {}", self.name);
        println!("> Inscription Scroll: {}", self.inscription);
    }
}