#[macro_use]
extern crate neon;
extern crate diskus;
extern crate humansize;
extern crate num_cpus;

mod disk_util;
mod fibo;
mod hello;

register_module!(mut cx, {
    cx.export_function("hello", hello::hello)?;
    cx.export_function("fibonacci", fibo::fibonacci)?;
    cx.export_function("fibonacciAsync", fibo::fibonacci_async)?;
    cx.export_function("dirSize", disk_util::dir_size)?;
    Ok(())
});
