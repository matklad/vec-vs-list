# `std::Vec` vs. `linked_list::LinkedList`

The benchmark creates 10_000 random 64 bit integers and then inserts them into
an empty container, maintaining the sorter order. With `LinkedList`, we don't
need to move elements when we insert into the middle of the collection. With
`Vec`, we utilize cache more efficiently.

Results on my machine:

```
❯ cargo run --release
    Finished release [optimized] target(s) in 0.0 secs
     Running `target/release/bench`
Generate random elements    744 μs
Collecting to sorted vector 33365 μs
Collecting to sorted list   184282 μs
```
