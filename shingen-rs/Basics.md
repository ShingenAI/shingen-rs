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

# 