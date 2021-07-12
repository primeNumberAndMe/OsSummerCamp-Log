struct Person{
    name: String,
    age: i32,
    height: i32,
    weight: i32,
}

impl Person{
    fn person_create(name: String, age: i32, height: i32, weight: i32)->Self{
        Person{name,age,height,weight}
    }

    fn person_destroy(self){}

    fn person_print(&self){
        println!("name: {}", self.name);
        println!("\tage: {}", self.age);
        println!("\theight: {}", self.height);
        println!("\tname: {}", self.weight);
    }
}


fn main() {
    // make two people structures
    let mut joe = Person::person_create("Joe Alex".into(), 32, 64, 140);
    let mut frank = Person::person_create("Frank Blank".into(), 20, 72, 180);


    // print them out and where they are in memory
    println!("Joe is at memory location {:p}: ", &joe);
    joe.person_print();
    println!("Frank is at memory location {:p}: ", &frank);
    frank.person_print();


    // make everyone age 20 years and print them again
    joe.age+=20;
    joe.height-=2;
    joe.weight+=40;
    joe.person_print();

    frank.age+=20;
    frank.weight+=20;
    frank.person_print();

    // destroy them both so we clean up
    Person::person_destroy(joe);
    Person::person_destroy(frank);
}


