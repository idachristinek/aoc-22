use rust2::calc1::score_game;
use rust2::calc2::score_game_2;
use rust2::file::read_file;

fn main() {
    let file_path = "src/input.txt";
    let content = read_file(file_path);
    let values = content.lines();

    let mut score = 0;
    let mut score_2 = 0;

    for value in values {
        let chunks = value.split(' ').collect::<Vec<&str>>();
        score += score_game(&chunks);
        score_2 += score_game_2(&chunks);
    }

    println!("Total score in round 1: {} ", score);
    println!("Total score in round 2: {} ", score_2);
}
