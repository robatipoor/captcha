extern crate captcha;
extern crate time;

use std::time::Instant;
use captcha::{gen, Difficulty};
use std::thread;

fn main() {
    let n = 20;
    let nthreads = 8;
    let mut threads = vec![];
    let now = Instant::now();

    for _ in 0..nthreads {
        let h = thread::spawn(move || {
            for _ in 0..n {
                gen(Difficulty::Easy).as_tuple();
            }
        });
        threads.push(h);
    }

    for i in threads {
        i.join().expect("join failed");
    }
    let elapsed =  now.elapsed().as_millis();
    println!("n                     : {}", n * nthreads);
    println!("time in ms total      : {}",elapsed);
    println!("time in ms per captcha: {}", elapsed / (n * nthreads));
    println!("captchs per second    : {}", n * nthreads * 1000 / elapsed);
}