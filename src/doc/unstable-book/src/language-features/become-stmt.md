# `become_stmt`

The tracking issue for this feature is: FIXME [#ISSUE]

[#ISSUE]: https://github.com/rust-lang/rust/issues/66666

------------------------

Currently, each function call might create a new stack frame.
This makes it impossible to have a long chain of deeply
recursive functions without exhausting the stack. While the
compiler might optimize this, it is in no way guaranteed.

The `become <expr>` forces the compiler to perform something called
tail-call, which does not increase stack space when performing
recursive function calls. The `<expr>` inside `become` statement
must always be a call to a function of same type (same arguments and return type).

Example:
```rust
#![feature(become_stmt)]

fn fib(n: usize, left: usize, right: usize) -> usize {
    match n {
        0 => 0,
        1 => b,
        rec => become fib(n - 1, right, left + right)
    }
}

fn main() {
    let value = fib(30, 0, 1);
    assert_eq!(value, 514229);
}
```
