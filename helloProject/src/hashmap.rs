// HASHMAP 
fn main() {
    println!("full_message");

    // HASHMAPS
    use std::collections::HashMap;
    let mut score: HashMap<String, i32> = HashMap::new();

    score.insert(String::from("Blue"), 10);
    score.insert(String::from("Yellow"), 60);

    let team_name: String = String::from("Blue");
    let team_score: i32 = score.get(&team_name).copied().unwrap_or(0);

    println!("Score for {}: {}", team_name, team_score);

    for (key, value) in &score {
        println!("{key}: {value}");
    }
}