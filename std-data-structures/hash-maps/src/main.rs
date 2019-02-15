use std::collections::HashMap;

fn main() {
    let mut empty_map = HashMap::new(); // create empty hashmap
    empty_map.insert(String::from("key1"), 32); //insert key-val pair into hashmap

    let mut scores = HashMap::new();
    scores.insert(String::from("Red"), 10);
    scores.insert(String::from("Blue"), 50);

    let teams = vec![String::from("Red"), String::from("Blue")];
    let tscores = vec![10, 50];
    let hashscores: HashMap<_, _> = teams
        .iter()
        .zip(tscores.iter()) //turn to iterators into one which holds tuples of values
        //(val_from_first_iter, val_from_second_iter)
        .collect(); //collect into hashmap

    let _name = String::from("Blue");
    let _get_score = hashscores.get(&_name); //get score-value of blue team
    println!("The score of team {} is {:?}", _name, _get_score);

    //iterate over key-val in hashmap
    for (key, val) in scores {
        println!("{} : {}", key, val);
    }

    let mut new_scores = HashMap::new();
    new_scores.insert(String::from("Red"), 10);
    new_scores.insert(String::from("Blue"), 50);

    new_scores.insert(String::from("Blue"), 25); //override key "Blue"

    {
        //only insert value if key doesnt exist
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        println!("{:?}", scores);
    }
}
