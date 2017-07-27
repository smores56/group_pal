extern crate group_pal;

use group_pal::config;

fn main() {
    let conf = config::load_conf();
    println!("{}", conf.id());
    println!("{:?}", conf.matches());
}
