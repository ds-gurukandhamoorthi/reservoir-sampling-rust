extern crate rand;


use rand::distributions::{Distribution, Uniform};

fn main() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10].iter();

    let mut rng = rand::thread_rng();

    let k = 3;
    let mut res = Vec::new();
    for (i, val) in a.enumerate() {
        if i < k {
            res.push(val);
        } else {
            let j = Uniform::from(0..=i).sample(&mut rng);
            if j < k {
                res[j] = val;
            }
        }
    }
    println!("{:?}", res);
}
