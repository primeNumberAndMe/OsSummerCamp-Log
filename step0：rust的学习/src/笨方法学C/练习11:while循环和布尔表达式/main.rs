use std::env;

fn main() {
    let argv = env::args().collect::<Vec<String>>();
    let argc = argv.len();
    let mut i = 0usize;
    while i < argc {
        println!("arg {}: {}", i, argv[i]);
        i+=1;
    }

    let states = ["California", "Oregon", "Washington", "Texas"];
    i = 0;
    while i < 4 {
        println!("state {}: {}", i, states[i]);
        i+=1;
    }
}


