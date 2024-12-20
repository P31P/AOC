use std::io;

fn main() {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let mut t1: i32 = iter.next().unwrap().parse().unwrap();
    let mut t2: i32 = iter.next().unwrap().parse().unwrap();

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
    let mut count;
    for num1 in &v1{
        count=0;
        for num2 in &v2{
            if num1==num2 {
                count=count+1;
            }
        }
        sum+=count*num1;
    }
    println!("{}", sum);
}
