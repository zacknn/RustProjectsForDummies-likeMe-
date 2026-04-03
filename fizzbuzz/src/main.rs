fn fizzbuzz(count: i32) {
    for i in 1..=count {
        match i {
            i if i % 15 == 0 => println!("fizzbuzz"),
            i if i % 3 == 0 => println!("fizz"),
            i if i % 5 == 0 => println!("buzz"),
            _ => println!("{}", i),
        }
    }
}

fn main() {
    let count = 15;
    fizzbuzz(count);
}

