use std::env;


fn main() {
    let mut count = 1;
    let mut arg_iter = env::args();
    arg_iter.next();
    for item in arg_iter{
        println!("arg {}: {}", count, item);
        count+=1;
    }

    let mut ano_count = 1;
    let states = [ "California", "Oregon", "Washington", "Texas"];
    for state in states{
        println!("state {}: {}", ano_count, state);
        ano_count+=1;
    }
}
