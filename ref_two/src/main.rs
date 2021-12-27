fn main() {
    let mut s = String::from("hello");
    {
        let s1 = &mut s;
        println!("ref_1:{}",s1);
    }
    let s2 = &mut s;
    println!("ref_2:{}",s2);
}
