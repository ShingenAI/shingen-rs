mod rituals;
use rituals::invoke::listen;
use rituals::form::DragonForm;

fn main() {
    let mut dragon = DragonForm::new("Shinryū", 9999, "Soul FLame");
    dragon.print_status();
    listen(&mut dragon);
    dragon.ignite(1);
    dragon.ignite(5);
    dragon.ignite(10);
    dragon.print_status();
    dragon.log_ignites();
}
