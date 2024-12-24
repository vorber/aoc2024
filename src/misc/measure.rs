use std::time::{Duration, Instant};

pub fn measure<T,U>(f:impl Fn(T)->U, arg:T) -> (U, Duration) {
    let start = Instant::now();
    let res = f(arg);
    (res, start.elapsed())
}

