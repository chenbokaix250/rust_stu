const MAX_POINTS:u32 = 100_000;
fn main() {
    println!("Hello, world!");

    let mut x =5;
    println!("The calue of x is {}",x);

    x = 6;
    println!("The calue of x is {}",x);
    println!("value is {}",MAX_POINTS);

    let guess:u32 = "42".parse().expect("Not a nunber");
    println!("{}",guess);

    let xx = 'z';
    let y:char = '≠';
    let z = '😊';
    println!("{}",z);
}
