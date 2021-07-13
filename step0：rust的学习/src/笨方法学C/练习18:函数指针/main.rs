use std::env;


type CompareCb = fn(i32, i32)->i32;

fn bubble_sort(mut numbers: Vec<i32>, cmp: CompareCb)->Vec<i32>{
    let len = numbers.len();
    for i in 0..len {
        for j in 0..len-i-1 {
            if cmp(numbers[j], numbers[j+1]) > 0 {
                let temp = numbers[j];
                numbers[j] = numbers[j+1];
                numbers[j+1] = temp;
            }
        }
    }
    numbers
}


fn sorted_order(a: i32, b: i32)->i32{
    a-b
}

fn reverse_order(a: i32, b: i32)->i32{
    b-a
}

fn strange_order(a: i32, b: i32)->i32{
    if a == 0 || b == 0 {
        return 0;
    }else{
        return a%b;
    }
}

fn test_sorting(numbers: Vec<i32>, cmp: CompareCb){
    let results = bubble_sort(numbers, cmp);
    for num in results.iter(){
        print!("{} ", num);
    }
    print!("\n");
}

fn main(){
    let mut ite = env::args();
    ite.next();
    let numbers = ite.map(|x| x.parse().unwrap()).collect::<Vec<i32>>();
    test_sorting(numbers.clone(), sorted_order);
    test_sorting(numbers.clone(), reverse_order);
    test_sorting(numbers.clone(), strange_order);
}
