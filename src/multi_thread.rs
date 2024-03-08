use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

pub fn multi_thread(){
    let img = image::open(
        "./test/majestic-mountain-peak-tranquil-winter-landscape-generated-by-ai.jpg",
    )
    .unwrap();
    let img = Arc::new(Mutex::new(img));
    let pool = ThreadPool::new(10);
    for i in 0..10 {
        let img_clone = Arc::clone(&img);
        pool.execute(move || {
            let img = img_clone.lock().unwrap();
            let resized_img = img.resize(1920, 1080, image::imageops::FilterType::Triangle);
            resized_img.save(format!("./test/output/output_{}.avif", i)).unwrap();
        });
    }
    pool.join();

}