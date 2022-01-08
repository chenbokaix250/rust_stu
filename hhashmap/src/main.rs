use std::collections::HashMap;

fn main() {
    let teams = vec![String::from("blue"),String::from("Yellow")];
    let intial_scores = vec![10,50];

    let scores:HashMap<_,_> = teams.iter().zip(intial_scores.iter()).collect();

    let team_name= String::from("blue");
    let score = scores.get(&team_name);
    //println!("{}",sco1);

    match score {
        Some(s)=>println!("{}",s),
        None => println!("team not exist"),
    };
}
