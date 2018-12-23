use std::collections::HashMap;

fn main () {
    // HashMap<K, V> stores Keys of type K and Values of type V
    let mut scores = HashMap::new();

    // inserting individual values in a mutable hashmap
    scores.insert("blue".to_string(), 10);
    scores.insert("yellow".to_string(), 50);
    
    // creating a hashmap from pairs of keys and values
    let teams = vec!["blue".to_string(), "yellow".to_string()];
    let initial_scores = vec![10, 50];
    // This method seems to create references... 
    let scores: HashMap<_, _> = teams // rust infers the types at this point
                                .iter()
                                .zip(initial_scores.iter())
                                .collect();
    println!("{:?}", scores); 
    // HashMaps take ownership of things that are put in them. While integers
    // and the like are copied over, strings and structs will become invalidated
    // once they move into the hashmap
    let question = "Got Bieber fever?".to_string();
    let answer   = "Don't we all?".to_string();
    println!("Q: {} A: {}", question, answer);
    let what = "Favorite Beatle?".to_string();
    let yeah = "Sir Paul".to_string();
    println!("Q: {} A: {}", what, yeah);

    let mut hizz = HashMap::new();
    hizz.insert(question, answer); // question and answer are now invalid
    hizz.insert(what, yeah);       // what and yeah are now invalid
    // Printing the q and a here will result in an error

    // Accessing values -------------------------------------------------------
    // Values can be accessed by using the get method:
    let question = "Got Bieber fever?".to_string();
    let answer = hizz.get(&question); // get returns Option<&V>
    // https://www.ameyalokare.com/rust/2017/10/23/rust-options.html
    println!("Q: {} A: {}", question, answer.unwrap_or(&"<crickets>".to_owned()));
    let question = "Do you want some medicine for that?".to_string();
    let answer = hizz.get(&question);
    println!("Q: {} A: {}", question, answer.unwrap_or(&"<crickets>".to_owned()));

    // Iterating over keypairs ------------------------------------------------
    // This is similar to perl
    for (key, value) in &hizz {
        println!("Q: {} A: {}", key, value);
    }


    // Replacing values in a hashmap -------------------------------------------
    let mut scores = HashMap::new();
    // insert method
    scores.insert("red".to_string(), 25);
    scores.insert("red".to_string(), 10);
    println!("{:?}", scores); // {"red": 10}
    // entry method: check if an entry exists and update it if it doesn't,
    scores.entry("blue".to_string()).or_insert(25);
    scores.entry("red".to_string()).or_insert(25);
    println!("{:?}", scores); // {"red": 10, "blue": 25}


    // Updating values in hashmaps based on old values -------------------------
    let text = "hello my baby hello my honey hello my ragtime gal";
    let mut words = HashMap::new();

    for word in text.split_whitespace() {
        let count = words.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{}\n{:?}", text, words);
}
