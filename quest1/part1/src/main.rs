fn main() {
    let input = include_str!("../input.txt");
    let sum: i32 = input
        .chars()
        .map(|c| match c {
            'A' => 0,
            'B' => 1,
            'C' => 3,
            _ => 0,
        })
        .sum();
    println!("{sum}");
}
