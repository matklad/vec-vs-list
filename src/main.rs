extern crate rand;
extern crate linked_list;

use std::time::Instant;
use std::iter::FromIterator;
use linked_list::LinkedList;

fn main() {
    let xs = time_it("Generate random elements   ", || elements());
    let vs = time_it("Collecting to sorted vector", || process_vec(&xs));
    let ls = time_it("Collecting to sorted list  ", || process_linked_list(&xs));

    let ls = Vec::from_iter(ls.into_iter());
    assert_eq!(ls, vs);
}

fn process_vec(xs: &[u64]) -> Vec<u64> {
    let mut result = vec![];
    for &x in xs {
        let i = result.iter().position(|&y| x <= y).unwrap_or(result.len());
        result.insert(i, x);
    }
    result
}

fn process_linked_list(xs: &[u64]) -> LinkedList<u64> {
    let mut result = LinkedList::new();
    for &x in xs {
        let mut c = result.cursor();
        loop {
            match c.peek_next() {
                Some(&mut y) if x <= y => { break; }
                None => { break; }
                Some(_) => {}
            }
            c.next();
        }
        c.insert(x)
    }
    result
}

fn elements() -> Vec<u64> {
    let n = 10000;
    (0..n).map(|_| rand::random::<u64>()).collect()
}

fn time_it<R, F: FnOnce() -> R>(tag: &str, f: F) -> F::Output {
    let start = Instant::now();
    let result = f();
    let duration = Instant::now() - start;
    assert!(duration.as_secs() == 0);
    println!("{} {} μs", tag, duration.subsec_nanos() / 1000);
    result
}