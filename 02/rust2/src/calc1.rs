fn did_win(player1: &char, player2: &char) -> i32 {
    if player1 == &'A' {
        return match player2 {
            'X' => 3,
            'Y' => 6,
            'Z' => 0,
            &_ => 0,
        };
    }
    if player1 == &'B' {
        return match player2 {
            'X' => 0,
            'Y' => 3,
            'Z' => 6,
            &_ => 0,
        };
    }
    if player1 == &'C' {
        return match player2 {
            'X' => 6,
            'Y' => 0,
            'Z' => 3,
            &_ => 0,
        };
    }
    return 0;
}

pub fn score_game(values: &Vec<&str>) -> i32 {
    let value1: char = values[0].chars().next().unwrap();
    let value2: char = values[1].chars().next().unwrap();

    let points1 = did_win(&value1, &value2);
    let points2 = added_points(&value2);

    return points1 + points2;
}

fn added_points(player2: &char) -> i32 {
    return match player2 {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        &_ => 0,
    };
}
