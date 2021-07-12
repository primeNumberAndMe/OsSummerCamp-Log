use std::env;


fn main() {
    let argv = env::args().collect::<Vec<String>>();
    let argc = argv.len();
    if argc != 2 {
        println!("ERROR: You need one argument.");
        return;
    }
    let v = argv[1].as_bytes().iter().map(|&x| x as char).enumerate().collect::<Vec<(usize, char)>>();
    for (i, char) in v.iter(){
        match &char {
            'a' | 'A' => {
                println!("{}: 'A'", i);
            },
            'e' | 'E' => {
                println!("{}: 'E'", i);
            },
            'i' | 'I' => {
            println!("{}: 'I'", i);
            },
            'o' | 'O' => {
                println!("{}: 'O'", i);
            },
            'u' | 'U' => {
                println!("{}: 'U'", i);
            },
            'y' | 'Y' if *i > 2 => {
                println!("{}: 'Y'", i);
            },
            _ => {
                println!("{}: {} is not a vowel", i, char);
            },
        }
    }
}


