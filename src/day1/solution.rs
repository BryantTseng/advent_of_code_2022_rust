use std::fs;
use std::str::FromStr;
pub fn solution() -> i32 {
    let contents =
        fs::read_to_string("input/day1.txt").expect("Should have been able to read the file");
    let lines = contents.lines();
    let mut ans = 0;
    let mut temp = 0;
    for each in lines {
        if each == "" {
            if temp > ans {
                ans = temp;
            }
            temp = 0;
        } else {
            let k: i32 = FromStr::from_str(each).unwrap();
            temp = temp + k;
        }
    }
    if temp > ans {
        ans = temp;
    }

    ans
}
