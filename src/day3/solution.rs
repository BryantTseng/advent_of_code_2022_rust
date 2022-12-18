use std::fs;

pub fn solution() -> u32 {
    let contents =
        fs::read_to_string("input/day3.txt").expect("Should have been able to read the file");
    let mut total_point = 0;
    for line in contents.lines() {
        let first_part = line[0..line.len() / 2].to_string();
        let second_part = line[line.len() / 2..].to_string();
        for each in first_part.chars() {
            if second_part.contains(each) {
                if each.is_ascii_uppercase(){
                    let p = each as u32 - 65 + 27;
                    total_point+=p;
                }else {
                    let p = each as u32 - 97 + 1;
                    total_point+=p;

                }
                break;
            }
        }
    }

    total_point
}
