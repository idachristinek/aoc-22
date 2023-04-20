use rust5::file::read_file;

fn main() {
    let file_path = "src/input-test.txt";
    let content = read_file(file_path);
    let values = content.lines();
    let mut cargo: Vec<&str> = Vec::new();
    let mut instructions: Vec<&str> = Vec::new();

    for value in values {
        if value.is_empty() {
        } else if value.chars().nth(0) == Some('m') {
            instructions.push(value);
        } else {
            cargo.push(value);
        }
    }

    // check last number in last line in carg to see how many we have
    let x = cargo.last().unwrap();
    let y: Vec<char> = x.chars().collect();
    let num = y.get(y.len() - 2).unwrap();
    let size = num.to_digit(10).unwrap();

    let mut main_cargo: Vec<Vec<char>> = Vec::with_capacity(size.try_into().unwrap());
    for _ in 0..size {
        let mut col = Vec::with_capacity(cargo.len());
        for _ in 0..cargo.len() {
            col.push(' ');
        }
        main_cargo.push(col);
    }

    let mut j = 0;

    println!("Cargo:");
    while j < cargo.len() {
        println!("{:?}", cargo.get(j));

        let mut i = 0;
        let mut k = 0;
        let mut last_index_added = 0;
        let buffer = 4;
        let letters: Vec<char> = cargo.get(j).expect("REASON").chars().collect();

        while i < letters.len() {
            let working_part: char = *letters.get(i).unwrap();

            if i == 1 {
                last_index_added = i;
                main_cargo[0][j] = working_part;
                k += 1;
            } else if i > 2 && i < letters.len() - 2 && i == last_index_added + buffer {
                last_index_added = i;
                main_cargo[k][j] = working_part;
                k += 1;
            } else if i == letters.len() - 2 {
                main_cargo[k][j] = working_part;
                last_index_added = i;
                k = 0;
            }

            i = i + 1;
        }
        j += 1;
    }

    println!("{:?}", main_cargo);
    println!("Instructions:");
    for value in instructions {
        println!("{:?}", value);
    }
}
