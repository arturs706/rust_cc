
mod multi_thread;
use multi_thread::multi_thread;
use single_thread::single_thread;
mod single_thread;
mod disruptor_test;
use disruptor_test::disruptor_fn;
use std::time::Instant;
mod crossbeam_test;
use crossbeam_test::crossbeam_resize_image;


fn main() {
    let start_time = Instant::now();
    disruptor_fn();
    println!("Time taken to process {:?}", start_time.elapsed().as_secs_f64());
}
