use itertools::Itertools;

#[aoc::main(1)]
fn main(input: &str) -> (usize, usize) {
    let vals = input
        .trim()
        .split("\n\n")
        .map(|elf| {
            elf.lines().map(|val| {
                val.parse().unwrap_or(0)
            }).sum()
        })
        .sorted()
        .rev()
        .collect::<Vec<_>>();
    (vals[0], vals[0] + vals[1] + vals[2])
}