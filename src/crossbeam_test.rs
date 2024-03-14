use crossbeam_channel::bounded;
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

pub fn crossbeam_resize_image() {
    let (snd, rcv) = bounded(10);
    let img =
        image::open("./test/majestic-mountain-peak-tranquil-winter-landscape-generated-by-ai.jpg")
            .unwrap();
    let img = Arc::new(Mutex::new(img));
    let pool = ThreadPool::new(10);

    for _ in 0..10 {
        let snd_clone = snd.clone();
        let img_clone = Arc::clone(&img);
        pool.execute(move || {
            let resized_img =
                img_clone
                    .lock()
                    .unwrap()
                    .resize(1920, 1080, image::imageops::FilterType::Triangle);
            snd_clone.send(resized_img).unwrap();
        });
    }

    let saving_pool = ThreadPool::new(10);

    for i in 0..10 {
        let received_img = rcv.recv().unwrap();
        let img_path = format!("./test/output/output_{}.avif", i);
        let received_img_clone = received_img.clone();
        saving_pool.execute(move || {
            received_img_clone.save(img_path).unwrap();
        });
    }

    pool.join();
    saving_pool.join();
}
