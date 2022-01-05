# rust-calc

A rust calculator that can perform a bunch of arithmetic operations. I wrote
this program as part of my learning of important concepts like:

* [Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
* [Structs](https://doc.rust-lang.org/book/ch05-00-structs.html)
* [Enums](https://doc.rust-lang.org/book/ch06-00-enums.html)
* [Iterator](https://doc.rust-lang.org/book/ch13-02-iterators.html)
* [Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
* [Casting](https://doc.rust-lang.org/stable/rust-by-example/conversion.html)
* [Unit tests](https://doc.rust-lang.org/book/ch11-00-testing.html)

To run the program:

```sh
➜ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/rust-calc`
Type '?' to see supported operators, 'q' to quit
# ?
Supported operators: [+,-,*,/,]
# 1.0+2.0
> 3
# 3.0-9.0
> -6
# 4.5  / 4.5
> 1
# 6.0 / 3.0
> 2
# 6.0*9.0
> 54
# 1.3456*9.728
> 13.089996799999998
# q
➜
```

To run the unit tests:

```sh
cargo test
```
