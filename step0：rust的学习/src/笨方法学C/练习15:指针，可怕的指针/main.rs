use std::mem;


fn main() {
    // create two arrays we care about
    let ages = [23, 43, 12, 89, 2];
    let names = [
        "Alan", "Frank",
        "Mary", "John", "Lisa"
    ];

    // safely get the size of ages
    let count = mem::size_of_val(&ages) / mem::size_of::<i32>();

    // first way using indexing
    for i in 0..count{
        println!("{} has {} yeaers alive", names[i], ages[i]);
    }
    println!("---");

    // setup the pointers to the start of the arrays
    let mut cur_age = ages.as_ptr();
    let mut cur_name = names.as_ptr();

    // second way using pointers
    for i in 0..count{
        unsafe{
            println!("{} has {} yeaers alive", *(cur_name.add(i)), *(cur_age.add(i)));
        }
    }
    println!("---");

    // fourth way with pointers in a stupid complex way
    cur_age = ages.as_ptr();
    cur_name = names.as_ptr();
    while (cur_age as usize - ages.as_ptr() as usize)/mem::size_of::<i32>() < count {
        unsafe{
            println!("{} has {} yeaers alive", *(cur_name), *(cur_age));
            cur_age=cur_age.add(1);
            cur_name=cur_name.add(1);
        }
    }
}


