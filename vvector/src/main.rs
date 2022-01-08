fn main() {
    let v= vec![1,2,3,4,5];
    let third:&i32 = &v[2];
    println!("the 3rd is {}",third);

    match v.get(30){
        Some(third)=>println!("{}",third),
        None=>println!("None"),
    }
}

