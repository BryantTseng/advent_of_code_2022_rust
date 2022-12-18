use std::fs;
pub fn solution() -> i32 {
    let contents =
        fs::read_to_string("input/day2.txt").expect("Should have been able to read the file");

    let mut total_point = 0;

    for each in contents.lines() {
        let mut each_iter = each.split_whitespace();
        let opponent = each_iter.next().unwrap();
        let mine = each_iter.next().unwrap();

        let mut point_this_round = 0;
        match mine {
            "X" => {
                point_this_round += 1;
                if opponent == "A" {
                    point_this_round += 3;
                } else if opponent == "B" {
                    point_this_round += 0;
                } else if opponent == "C" {
                    point_this_round += 6;
                }
            }
            "Y" => {
                point_this_round += 2;
                if opponent == "A" {
                    point_this_round += 6;
                } else if opponent == "B" {
                    point_this_round += 3;
                } else if opponent == "C" {
                    point_this_round += 0;
                }
            }
            "Z" => {
                point_this_round += 3;
                if opponent == "A" {
                    point_this_round += 0;
                } else if opponent == "B" {
                    point_this_round += 6;
                } else if opponent == "C" {
                    point_this_round += 3;
                }
            }
            _ => {}
        }
        total_point += point_this_round;
    }
    total_point
}
