use rust3::file::read_file;

fn main() {
    let file_path = "src/input.txt";
    let content = read_file(file_path);
    let values = content.lines();
    let mut duplicates: Vec<char> = Vec::new();
    let mut score: u32 = 0;
    let mut groups: Vec<Vec<&str>> = Vec::new();

    for value in values {
        let letter: char = find_duplicates(value);
        duplicates.push(letter);
    }

    for duplicate in &duplicates {
        score += score_chars(*duplicate);
    }

    println!("Total score: {}", score);

    // part 2
    let values_2 = content.lines();
    let mut elves: Vec<&str> = Vec::new();
    let mut badges: Vec<char> = Vec::new();
    let mut badge_score: u32 = 0;

    for value in values_2 {
        if elves.len() == 3 {
            groups.push(elves);

            elves = Vec::new();
            elves.push(value);
        } else {
            elves.push(value);
        }
    }
    //add last
    groups.push(elves);

    for group in groups {
        badges.push(find_badge_for_group(group));
    }

    for b in &badges {
        badge_score += score_chars(*b);
    }
    println!("Total badge score: {}", badge_score);
}

fn find_duplicates(input: &str) -> char {
    let place_to_split: usize = input.len() / 2;
    let (first, last) = input.split_at(place_to_split);

    for letter in first.chars() {
        if last.contains(letter) {
            return letter;
        }
    }
    return '0';
}

fn find_badge_for_group(mut group_contents: Vec<&str>) -> char {
    group_contents.sort_by(|a, b| a.len().cmp(&b.len()));

    let first = group_contents[0];
    let second = group_contents[1];
    let third = group_contents[2];
    for x in first.chars() {
        let in_second = second.contains(x);
        let in_third = third.contains(x);

        if in_second && in_third {
            return x;
        }
    }
    return '0';
}

fn score_chars(letter: char) -> u32 {
    let numeric_rep = letter as u32;

    //a-z
    if numeric_rep > 96 {
        return numeric_rep - 96;
    }
    {
        //A-Z
        return numeric_rep - 38;
    }
}
