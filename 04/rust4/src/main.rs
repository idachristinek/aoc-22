use rust4::file::read_file;
use std::collections::HashSet;

fn main() {
    let file_path = "src/input.txt";
    let content = read_file(file_path);
    let values = content.lines();

    for value in values {
        println!("{}", value);

        let parts: Vec<&str> = value.split(',').collect();
        println!("first: {}, last: {}", parts[0], parts[1]);

        let test:HashSet<i32> =  fill_hash(parts[0]);
        println!("{:?}", test);
    }
}

fn fill_hash(input: &str) -> HashSet<i32> {
    let mut data = HashSet::new();
    let parts: Vec<&str> = input.split('-').collect();
 let start = parts[0].parse::<i32>().unwrap();

 let end = parts[1].parse::<i32>().unwrap();

 println!("start: {}, end {}", start, end);

 for i in start..end+1{
    data.insert(i);
 }

 return data;
    
}
