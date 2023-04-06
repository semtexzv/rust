// This test makes sure that

// edition:2018
// compile-flags: -C panic=abort

#![crate_type = "lib"]
#![feature(become_stmt)]

// EMIT_MIR become_stmt.f1.0.mir
#[inline(never)]
fn f1(a: u32, b: u32) -> u32 {
    if b > a {
        become f2(a, b)
    } else {
        become f3(a, b)
    }
}

// EMIT_MIR become_stmt.f2.0.mir
#[inline(never)]
fn f2(mut a: u32, b: u32) -> u32 {
    if a > b {
        a += 1;
    };
    become f3(a, b)
}

// EMIT_MIR become_stmt.f3.0.mir
#[inline(never)]
fn f3(a: u32, b: u32) -> u32 {
    std::cmp::max(a, b)
}
