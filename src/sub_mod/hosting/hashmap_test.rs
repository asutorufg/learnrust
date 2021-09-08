use std::collections::HashMap;

pub fn hashmap() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores: {:?}", scores);

    let key = vec![String::from("Blue"), String::from("Yellow")];
    let value = vec![10, 50];
    let ha: HashMap<_, _> = key.iter().zip(value.iter()).collect();
    println!("ha: {:?}", ha);

    let score = ha.get(&String::from("Blue"));
    println!("Blue: {:?}", score);

    for (key, value) in &ha {
        println!("{}: {}", key, value);
    }

    scores.insert(String::from("Blue"), 20);
    println!("scores: {:?}", scores);

    scores.entry(String::from("Blue")).or_insert(30);
    println!("scores: {:?}", scores);

    let c = scores.entry(String::from("Blue")).or_insert(0);
    *c += 1;
    println!("scores: {:?}", scores);
}
