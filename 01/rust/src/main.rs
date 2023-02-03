//use std::env;
use std::fmt;
use std::fs;

fn main() {
    let file_path = "src/input.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let values = contents.lines();

    //println!("With text:\n{contents}");

    let mut elves: Vec<Elf> = Vec::new();
    let mut idx = 1;
    let mut current_elf = Elf::new(idx, Vec::new(), 0);
    let mut max_elf = Elf::new(idx, Vec::new(), 0);

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

    for elf in elves {
        println!("Elf number {} has calories: {}", elf.number, elf.calories,);

        if max_elf.calories < elf.calories {
            println!(
                "Updating max_elf as current elf has more: curr: {}, max: {}",
                elf.calories, max_elf.calories
            );

            max_elf = Elf::new(elf.number, elf.values, elf.calories);
        }
    }
    println!(
        "The elf with the most calories: {} is number: {}",
        max_elf.calories, max_elf.number
    );

}

pub struct Elf {
    pub number: i32,
    pub values: Vec<String>,
    pub calories: i32,
}

impl Elf {
    pub fn new(number: i32, values: Vec<String>, calories: i32) -> Self {
        Self {
            values: values,
            calories: calories,
            number: number,
        }
    }
}

impl fmt::Debug for Elf {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Elf")
            .field("values", &self.values)
            .finish()
    }
}

impl fmt::Display for Elf {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut str = "Elf: {number} has {calories} calories. ";
        for name in &self.values {
            fmt.write_str(str)?;
            fmt.write_str(name)?;
            str = ", ";
        }
        Ok(())
    }
}
