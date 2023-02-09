fn did_win_2(player1: &char, player2: &char) -> i32 {
    if player1 == &'A' {
        return match player2 {
            'A' => 3,
            'B' => 6,
            'C' => 0,
            &_ => 0,
        };
    }
    if player1 == &'B' {
        return match player2 {
            'A' => 0,
            'B' => 3,
            'C' => 6,
            &_ => 0,
        };
    }
    if player1 == &'C' {
        return match player2 {
            'A' => 6,
            'B' => 0,
            'C' => 3,
            &_ => 0,
        };
    }
    return 0;
}

fn choose_shape(player1: &char, player2: &char) -> char {
    if *player1 == 'A' {
        return match player2 {
            'X' => 'C',
            'Y' => 'A',
            'Z' => 'B',
            &_ => 'Q',
        };
    }
    if *player1 == 'B' {
        return match player2 {
            'X' => 'A',
            'Y' => 'B',
            'Z' => 'C',
            &_ => 'Q',
        };
    }
    if *player1 == 'C' {
        return match player2 {
            'X' => 'B',
            'Y' => 'C',
            'Z' => 'A',
            &_ => 'Q',
        };
    }
    return 'Q';
}

pub fn score_game_2(values: &Vec<&str>) -> i32 {
    let value1: char = values[0].chars().next().unwrap();
    let value2: char = values[1].chars().next().unwrap();

    let action = choose_shape(&value1, &value2);

    let points1 = did_win_2(&value1, &action);
    let points2 = added_points(&action);

    return points1 + points2;
}

fn added_points(player2: &char) -> i32 {
    return match player2 {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        &_ => 0,
    };
}
