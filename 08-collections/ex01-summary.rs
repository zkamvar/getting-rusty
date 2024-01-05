use std::collections::HashMap;
use std::io;
use std::str::FromStr;

fn main() {
    let mut s = String::new();
    // shuf -r -i 1-10 -n 10 | tr '\\n' ' ' | ./ex01-summary
    io::stdin().read_line(&mut s).expect("I got nothing");
    // let s = "5 4 2 3 2 2 2 2 1 5 2 50".to_string();

    let mut v = Vec::new();
    for number in s.split_whitespace() {
        v.push(i32::from_str(number).unwrap());
    }
    // println!("{:?}", v);
    let (mean, median, mode) = mean_median_mode(&v);
    v.sort_unstable();
    println!("mean: {}, median: {}, mode: {}", mean, median, mode);
}

fn mean_median_mode(v: &Vec<i32>) -> (f64, i32, i32) {
    let mut modemap = HashMap::new();
    let denom = v.len() as f64;
    let mut numerator = 0;
    let med = (denom / 2.0).floor() as usize;
    for i in v {
        numerator += i;
        let count = modemap.entry(i).or_insert(0);
        *count += 1;
    }
    let mut mvec: Vec<_> = modemap.iter().collect();
    mvec.sort_by(|a, b| a.1.cmp(b.1));
    ((numerator as f64 / denom), v[med], **mvec[mvec.len() - 1].0)
}
