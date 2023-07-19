use getpairs::get_pairs;
use rand::Rng;

mod getpairs;
fn main() {
    let pairs = get_pairs();
    let index = rand::thread_rng().gen_range(0..pairs.len());
    let target_pair = pairs[index];
    let target_port = target_pair.0;
    println!("{}", target_port);
}
