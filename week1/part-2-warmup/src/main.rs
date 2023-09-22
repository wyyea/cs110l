/* The following exercises were borrowed from Will Crichton's CS 242 Rust lab. */

use std::collections::HashSet;

fn main() {
    println!("Hi! Try running \"cargo test\" to run tests.");
}

fn add_n(v: Vec<i32>, n: i32) -> Vec<i32> {
    let mut nv: Vec<i32> = Vec::new();
    for i in v.iter(){
        nv.push(*i + n);
    }
    nv
}

fn add_n_inplace(v: &mut Vec<i32>, n: i32) {
    for i in 0..v.len(){
        v[i] = v[i] + n;
    }
}

fn dedup(v: &mut Vec<i32>) {
    let mut hs: HashSet<i32> = HashSet::new();
    let mut n = 0;
    let mut t = 0;
    for i in 0..v.len(){
        t = i - n;
        if !hs.contains(&v[t]){
            hs.insert(v[t]);
        }else{
            v.remove(t);
            n += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_n() {
        assert_eq!(add_n(vec![1], 2), vec![3]);
    }

    #[test]
    fn test_add_n_inplace() {
        let mut v = vec![1];
        add_n_inplace(&mut v, 2);
        assert_eq!(v, vec![3]);
    }

    #[test]
    fn test_dedup() {
        let mut v = vec![3, 1, 0, 1, 4, 4];
        dedup(&mut v);
        assert_eq!(v, vec![3, 1, 0, 4]);
    }
}
