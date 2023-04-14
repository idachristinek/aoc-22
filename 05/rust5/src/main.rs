use rust5::file::read_file;

fn main() {
    let file_path = "src/input.txt";
    let content = read_file(file_path);
    let values = content.lines();
    let mut cargo: Vec<&str> = Vec::new();
    let mut instructions: Vec<&str> = Vec::new();

    for value in values {
        println!("{}- len: {}", value, value.len());

        if value.is_empty() {
        } else if value.chars().nth(0) == Some('m') {
            instructions.push(value);
        } else {
            cargo.push(value);
        }
    }

    //let mut main_cargo: Vec<Vec<&str>> = Vec::new();

    //check last number in last line in carg to see how many we have
    let x = cargo.last().unwrap();
    let mut y: Vec<char> = x.chars().collect();
    let num = y.get(y.len()-2).unwrap();
    let size =  num.to_digit(10).unwrap();

    println!("last line? {:?}", x);
    println!("number? {:?}", size);

    println!("Cargo:");
 for value in cargo {

        println!("{:?}", value);

        let mut i = 0;
        let mut last_index_added = 0;
        let buffer = 4;
        let mut letters: Vec<char> = value.chars().collect();

        while i < letters.len() {
            println!("{:?}", letters.get(i).unwrap());

          if i == 1 || i == letters.len()-2 {
                println!(" adding first or last {:?} to list", letters.get(i).unwrap());
                last_index_added = i;
            }

             else if i > 2 && i < letters.len()-2 && i == last_index_added + buffer{
                println!(" i is now {:?}", i);
                println!(" would add {:?} to list", letters.get(i).unwrap());
                last_index_added = i;
            }

            i = i + 1;
        }
    }

    println!("Instructions:");
    for value in instructions {
        println!("{:?}", value);
    }
}
