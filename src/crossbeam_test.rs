use std::sync::Arc;
use image::imageops::FilterType;
use crossbeam_channel::bounded;
use std::path::PathBuf;

fn process_image(image_path: &PathBuf, output_path: PathBuf) {
    let img = image::open(image_path).expect("Failed to open image");
    let resized_img = img.resize(1920, 1080, FilterType::Triangle);
    resized_img.save(output_path).expect("Failed to save resized image");
}

pub fn crossbeam_resize_image() {
    let (snd, rcv) = bounded::<usize>(1); // Channel for task identifiers
    let image_path = Arc::new(PathBuf::from("./test/majestic-mountain-peak-tranquil-winter-landscape-generated-by-ai.jpg"));
    let iterations = 10;

    crossbeam::scope(|s| {
        // Producer thread
        s.spawn(|_| {
            for i in 0..iterations {
                snd.send(i).unwrap(); // Send iteration number as task identifier
            }
            drop(snd);
        });

        // Worker thread for processing the image
        let image_path_clone = Arc::clone(&image_path);
        s.spawn(move |_| {
            for i in rcv.iter() {
                let output_path = format!("./test/output/output_{}.avif", i);
                process_image(&image_path_clone, PathBuf::from(output_path));
            }
        });
    }).unwrap();
}
