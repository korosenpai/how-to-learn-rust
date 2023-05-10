// 8.3: Given a list of integers, use a vector and return the median (when sorted, the value in the middle position)
// and mode (the value that occurs most often; a hash map will be helpful here) of the list

use std::collections::HashMap;

fn main() {
    let mut inp: Vec<i32> = vec![7, 2, 5, 2, 9, 4, 7, 6, 10, 2];
    inp.sort();
    println!("{:?}", inp);
    
    println!("median: {:?}", median(&inp));
    
    println!("mode: {:?}", mode(&inp));
}

fn median(inp: &Vec<i32>) -> i32 {
    inp[inp.len() / 2]
}

fn mode(inp: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for elem in inp {
        let entry: &mut i32 = map.entry(elem.clone()).or_insert(0);
        *entry += 1;
    }

    let max_value: (&i32, &i32) = map.iter().max_by_key(|entry | entry.1).unwrap();
    *max_value.0
}