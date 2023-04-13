use rust4::file::read_file;
use std::collections::HashSet;

fn main() {
    let file_path = "src/input.txt";
    let content = read_file(file_path);
    let values = content.lines();
    let mut count_full = 0;
    let mut count_any = 0;

    for value in values {
        println!("{}", value);
        let parts: Vec<&str> = value.split(',').collect();

        let test: HashSet<i32> = fill_hash(parts[0]);
        let test_2: HashSet<i32> = fill_hash(parts[1]);

        let subset_1 = test.is_subset(&test_2);
        let subset_2 = test_2.is_subset(&test);

        if subset_1 || subset_2 {
            count_full += 1;
        }

        for value in test {
            if test_2.contains(&value) {
                count_any += 1;
                break;
            }
        }

    }

    println!("Found {:?} fully overlapping pairs", count_full);
    println!("Found {:?} overlapping pairs", count_any);
}

fn fill_hash(input: &str) -> HashSet<i32> {
    let mut data = HashSet::new();
    let parts: Vec<&str> = input.split('-').collect();
    let start = parts[0].parse::<i32>().unwrap();
    let end = parts[1].parse::<i32>().unwrap();

    for i in start..end + 1 {
        data.insert(i);
    }

    return data;
}
