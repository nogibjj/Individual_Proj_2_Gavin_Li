use std::collections::HashMap;

pub fn logic(numbers: Vec<i32>) -> Vec<(i32, u32)> {
    let mut frequencies = HashMap::new();
    for num in numbers {
        let frequency = frequencies.entry(num).or_insert(0);
        *frequency += 1;
    }
    let mut result = Vec::new();
    for (num, frequency) in frequencies {
        result.push((num, frequency));
    }
    result
}

pub fn string_to_list(string: String) -> Vec<i32> {
    let numbers: Vec<i32> = string.split(',').filter_map(|n| n.parse().ok()).collect();
    println!("{:?}", numbers);
    numbers
}
