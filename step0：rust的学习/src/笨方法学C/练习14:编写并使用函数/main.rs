use std::env;

fn can_it_print(ch: char)->bool{
    ch.is_ascii_alphabetic() || ch.is_ascii_digit()
}

fn print_letters(arg: &String){
    for char in arg.chars(){
        if can_it_print(char) {
            print!("'{}' == {} ", char, char as i8);
        }
    }
    print!("\n");
}

fn print_arguments(argc: usize, argv: Vec<String>){
    for i in 0..argc{
        print_letters(&argv[i]);
    }
}


fn main() {
    let argv = env::args().collect::<Vec<String>>();
    let argc = argv.len();
    print_arguments(argc, argv);
}


