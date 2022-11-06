// 0 1 1 2 3 5 8 13 21 ...

fn main() {
    println!("Hello, world!");

    println!("{}", fibonacci(128));
}

pub fn fibonacci(n: i32) -> u128 {
    if n < 0 {
        panic!("{} is negative!", n);
    } else if n == 0 {
        panic!("zero is not a right argument to fibonacci()!");
    } else if n == 1 {
        return 1;
    }

    let mut sum = 0;
    let mut last = 0;
    let mut curr = 1;
    for _i in 1..n {
        sum = last + curr;
        last = curr;
        curr = sum;
    }
    sum
}
