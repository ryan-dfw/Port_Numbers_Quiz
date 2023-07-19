use getpairs::get_pairs;
use printlist::printlist;

mod getpairs;
mod printlist;
fn main() {
    let pairs = get_pairs();
    printlist(&pairs);
}
