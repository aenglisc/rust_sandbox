use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![5, 4, 3, 2, 1];
    println!("{:?}", numbers);
    println!("{:?}", numbers[0]);

    numbers[4] = 10;
    numbers.push(10);
    numbers.push(10);
    numbers.pop();

    println!("{:?}", numbers);
    println!("Vector: {} bytes", mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[0..2];
    println!("{:?}", slice);

    for x in numbers.iter() {
        println!("{}", x);
    }

    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("{:?}", numbers);
}


