pub fn part1() -> String {
    let path = "input/day1.txt";
    let input = std::fs::read_to_string(path).unwrap();
    let list1 = Vec::new();
    let list2 = Vec::new();
    let (mut list1, mut list2) = input.lines().map(|line| line.split("   ").collect()).fold(
        (list1, list2),
        |(mut l1, mut l2), v: Vec<&str>| {
            l1.push(v[0]);
            l2.push(v[1]);
            (l1, l2)
        },
    );
    list1.sort();
    list2.sort();
    list1
        .iter()
        .zip(list2.iter())
        .fold(0, |sum, (&a, &b)| {
            sum + i32::abs(a.parse::<i32>().unwrap() - b.parse::<i32>().unwrap())
        })
        .to_string()
}

pub fn part2() -> String {
    let path = "input/day1.txt";
    let input = std::fs::read_to_string(path).unwrap();
    "".to_string()
}
