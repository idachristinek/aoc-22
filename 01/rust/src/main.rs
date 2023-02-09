use rust::elf::Elf;
use rust::file::read_file;
use std::str::Lines;

fn main() {
    let file_path = "src/input.txt";
    let content = read_file(file_path);
    let values = content.lines();
    let mut elves: Vec<Elf> = Vec::new();

    add_elves(values, &mut elves);
    find_max_elf(&mut elves);
    find_top_three(&mut elves);
}

fn add_elves(values: Lines<'_>, elves: &mut Vec<Elf>) {
    let mut idx = 1;
    let mut current_elf = Elf::new(idx, Vec::new(), 0);

    for part in values {
        if part.is_empty() {
            elves.push(current_elf);
            idx += 1;
            current_elf = Elf::new(idx, Vec::new(), 0);
        } else {
            current_elf.values.push(String::from(part));

            let input: i32 = String::from(part).trim().parse().expect("Wanted a number");
            current_elf.calories += input
        }
    }

    // Adding final elf since our data does not end with an whitespace
    elves.push(current_elf);
}

fn find_max_elf(elves: &mut Vec<Elf>) {
    let mut max_elf = Elf::new(0, Vec::new(), 0);

    for elf in elves {
        if max_elf.calories < elf.calories {
            max_elf = Elf::new(elf.number, Vec::new(), elf.calories);
        }
    }

    println!(
        "The elf with the most calories: {} is number: {}",
        max_elf.calories, max_elf.number
    );
}

fn find_top_three(elves: &mut Vec<Elf>) {
    let mut top_three: Vec<i32> = Vec::new();
    let mut sorted: Vec<i32> = Vec::new();
    let mut top_three_total = 0;

    for elf in elves {
        sorted.push(elf.calories);
    }

    sorted.sort_by(|a, b| b.cmp(a));

    for n in 0..3 {
        top_three.push(sorted[n]);
        top_three_total += sorted[n];
    }

    println!("Top Three:  {:?}", top_three);
    println!("Top Three total:  {}", top_three_total);
}
