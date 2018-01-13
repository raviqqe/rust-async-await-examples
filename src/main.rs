#![feature(proc_macro, conservative_impl_trait, generators)]

extern crate futures_await as futures;
extern crate futures_cpupool;
#[macro_use]
extern crate lazy_static;

use futures::prelude::*;
use futures_cpupool::*;

lazy_static! {
    static ref POOL: CpuPool = CpuPool::new_num_cpus();
}

#[async]
fn foo() -> Result<i32, i32> {
    loop {}
}

#[async]
fn bar() -> Result<i32, i32> {
    let f1 = foo();
    let f2 = foo();

    let c1 = POOL.spawn(f1);
    let c2 = POOL.spawn(f2);

    c1.wait()?;
    c2.wait()?;

    return Ok(1);
}

fn main() {
    POOL.spawn(bar()).wait();
}
