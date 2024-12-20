use std::io;

fn main() {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let mut t1: i32 = iter.next().unwrap().parse().unwrap();
    let mut t2: i32 = iter.next().unwrap().parse().unwrap();

    // Read input pairs until -1 -1 is encountered, we can manually enter at last
    while t1 != -1 && t2 != -1 {
        v1.push(t1);
        v2.push(t2);
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        iter = input.trim().split_whitespace();
        t1 = iter.next().unwrap().parse().unwrap();
        t2 = iter.next().unwrap().parse().unwrap();
    }
    v1.sort();
    v2.sort();
    let mut sum = 0;
    for i in 0..v1.len() {
        sum += (v1[i] - v2[i]).abs();
    }
    println!("{}", sum);
}
