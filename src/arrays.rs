use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [5, 4, 3, 2, 1];
    println!("{:?}", numbers);
    println!("{:?}", numbers[0]);

    numbers[4] = 10;
    println!("{:?}", numbers);
    println!("Array: {} bytes", mem::size_of_val(&numbers));

    let slice: &[i32] = &numbers[0..2];
    println!("{:?}", slice);
}

