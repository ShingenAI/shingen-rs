// ─────────────────────────────────────────────
//  RITUAL INVOKE – SHRINE LISTENING GATE
//  shingen-rs / rituals / invoke.rs
// ─────────────────────────────────────────────
use crate::rituals::form::DragonForm;
use std::env;

pub fn listen(dragon: &mut DragonForm){
    let args:Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        println!("📜 No invocation scroll received.");
        println!("Usage: cargo run <command>");
        println!("Example: cargo run First breath");
        // Early return
        return
    }

    match args[0].as_str() {
        "ignite" => {
            println!("args.len = {}", args.len());
            if args.len() < 2 {
                println!("⚠️ No ignite level provided.");
                // Early return
                return
            }
            println!("args[1] = {}", args[1]);
            let level = args[1].parse::<u16>().unwrap_or(1);
            println!("level = {}", level);
            dragon.ignite(level);
        }

        "rest" => {
            println!("💤 The dragon rests. Shrine enters silence.");
        }

        "echo" => {
            if args.len() > 1 {
                let echo = &args[1];
                println!("🔊  Shrine echo: {}", echo);
            }else{
                println!("⚠️ No echo content provided.");
            }
        }

        unknown => {
            println!("❓ Unknown ritual: {}", unknown);
        }
    }
}