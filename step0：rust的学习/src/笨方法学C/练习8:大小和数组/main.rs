use std::mem;

fn main(){
    let areas = [10, 12, 13, 14, 20];
    let name = ['Z', 'e', 'd', '\0'];
    let full_name = ['Z', 'e', 'd', ' ', 'A', '.', ' ', 'S', 'h', 'a', 'w', '\0'];
    
    println!("The size of an int: {}", mem::size_of::<i32>());
    println!("The size of areas: {}", mem::size_of_val(&areas));
    println!("The size of ints in areas: {}", mem::size_of_val(&areas)/mem::size_of::<i32>());
    println!("The first area is {}, the 2nd is {}", areas[0], areas[1]);
    println!("The size of a char {}", mem::size_of::<char>());
    println!("The size of name ([char;4]): {}", mem::size_of_val(&name));
    println!("The number of chars: {}", mem::size_of_val(&name)/mem::size_of::<char>());
    println!("The size of full_name ([char;12]): {}", mem::size_of_val(&full_name));
    println!("The number of chars: {}", mem::size_of_val(&full_name)/mem::size_of::<char>());
    println!("name=\"{:?}\" and full_name=\"{:?}\"", name, full_name);
}
