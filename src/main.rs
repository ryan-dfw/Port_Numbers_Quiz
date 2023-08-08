use getpairs::get_pairs;
use rand::Rng;
use std::io;

mod getpairs;
fn main() {
    let pairs = get_pairs();
    loop {
        let index = rand::thread_rng().gen_range(0..pairs.len());
        let target_pair = pairs[index];
        let target_port = target_pair.0;
        println!("{}", target_port);
        let protocol = match target_pair.2 {
            'T' => "TCP",
            'U' => "UDP",
            // see if there's an exception to throw here & stop the program
            // also ignore ascii case everywhere
            _ => "unknown",
        };
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                //correctChecker(); have this return a boolean BUT BRANCH THIS
                if input.trim().eq_ignore_ascii_case("q") {
                    break;
                }
                if input.trim().to_lowercase() == target_pair.1.trim().to_lowercase() {
                    println!("correct! over {}", protocol)
                } else {
                    println!(
                        "wrong; {} is {}.",
                        target_pair.0.to_string(),
                        target_pair.1.trim()
                    );
                    println!("please type the correct name");
                    input.clear();
                    match io::stdin().read_line(&mut input) {
                        Ok(_) => {
                            if input.trim().to_lowercase() == target_pair.1.trim().to_lowercase() {
                                println!("correct! over {}", protocol)
                            }
                        }
                        Err(error) => {
                            println!("Error reading input: {}", error);
                        }
                    }
                };
            }
            Err(error) => {
                println!("Error reading input: {}", error);
            }
        }
        println!("-");
        input.clear();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim().eq_ignore_ascii_case("q") {
                    break;
                }
            }
            Err(error) => {
                println!("Error reading input: {}", error);
            }
        }
    }
}
