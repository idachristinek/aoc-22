use rust3::file::read_file;


fn main() {
    let file_path = "src/input.txt";
    let content = read_file(file_path);
    let values = content.lines();
    let mut duplicates: Vec<char> = Vec::new();

    for value in values {
        println!("{}",value);
        let letter:char = find_duplicates(value);
        duplicates.push(letter);
    }


    println!("{:?}",duplicates);

}

fn find_duplicates(input: &str ) -> char {

    let place_to_split: usize = input.len() / 2;
    let (first, last) = input.split_at(place_to_split);

    for letter in first.chars() {
        if last.contains(letter) {
            return letter;
        }
    }


return '0';
}

fn score_chars(letter: &char) -> i32 {

    return 0;
}
