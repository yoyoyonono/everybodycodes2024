fn main() {
    let input = include_str!("../input.txt");
    let sum: i32 = input
        .chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|pair| {
            if pair.contains(&'x') {
                if pair[0] == pair[1] {
                    return 0;
                }
                return value(*pair.iter().filter(|c| c != &&'x').next().unwrap());
            }
            value(pair[0]) + value(pair[1]) + 2
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
