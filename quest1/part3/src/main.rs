use core::panic;

fn main() {
    let input = include_str!("../input.txt");
    let sum: i32 = input
        .chars()
        .collect::<Vec<char>>()
        .chunks(3)
        .map(|pair| {
            if pair.contains(&'x') {
                let num_x = pair.iter().filter(|c| c == &&'x').count();
                return match num_x {
                    1 => {
                        return (*pair)
                            .iter()
                            .filter(|c| c != &&'x')
                            .map(|x| value(*x))
                            .sum::<i32>()
                            + 2;
                    }
                    2 => {
                        return value(*pair.iter().filter(|c| c != &&'x').next().unwrap());
                    }
                    3 => 0,
                    _ => {
                        panic!();
                    }
                };
            }
            value(pair[0]) + value(pair[1]) + value(pair[2]) + 6
        })
        .map(|x| {
            println!("{x}");
            x
        })
        .sum();
    println!("{sum}");
}

fn value(c: char) -> i32 {
    match c {
        'A' => 0,
        'B' => 1,
        'C' => 3,
        'D' => 5,
        _ => 0,
    }
}
