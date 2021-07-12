use std::env;

fn main() {
    let argv = env::args().collect::<Vec<String>>();
    let argc = argv.len();
    let mut i = 0usize;
    if argc == 1 {
        println!("You only have one argument. You suck.");
    }else if argc > 1 && argc < 4 {
        println!("Here's your arguments:");
        for i in 0..argc {
            print!("{} ", argv[i]);
        }
        print!("\n");
    }else{
        print!("You have too many arguments. You suck.\n");
    }
}


