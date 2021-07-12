fn main() {
    let mut numbers  = [0;4];
    let mut name = ['a', '\0', '\0', '\0'];


    // first, print them out raw
    println!("numbers: {} {} {} {}",
           numbers[0], numbers[1],
           numbers[2], numbers[3]);

    println!("name each: {} {} {} {}\n",
           name[0], name[1],
           name[2], name[3]);

    println!("name: {}", name.iter().collect::<String>());

    // setup the numbers
    numbers[0] = 1;
    numbers[1] = 2;
    numbers[2] = 3;
    numbers[3] = 4;

    // setup the name
    name[0] = 'Z';
    name[1] = 'e';
    name[2] = 'd';
    name[3] = '\0';

    // then print them out initialized
    println!("numbers: {} {} {} {}",
           numbers[0], numbers[1],
           numbers[2], numbers[3]);

    println!("name each: {} {} {} {}",
           name[0], name[1],
           name[2], name[3]);

    // print the name like a string
    println!("name: {}", name.iter().collect::<String>());

    // another way to use name
    let another = "Zed";

    println!("another: {}", another);

    println!("another each: {} {} {} {}\n",
           &another[0..1], &another[1..2],
           &another[2..3], &another[3..]);
}
