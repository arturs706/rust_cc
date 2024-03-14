mod multi_thread;
use multi_thread::multi_thread;
use single_thread::single_thread;
mod disruptor_test;
mod single_thread;
use disruptor_test::disruptor_fn;
use std::time::Instant;
mod crossbeam_test;
use crossbeam_test::crossbeam_resize_image;
use csv::WriterBuilder;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
mod tokio_test;
use tokio_test::tokio_test;



#[tokio::main]

async fn main() {
    let start_time = Instant::now();
    // disruptor_fn();
    tokio_test().await;
    // multi_thread();
    // single_thread();
    println!(
        "Time taken to process {:?}",
        start_time.elapsed().as_secs_f64()
    );
    if let Err(err) = delete_all_images("./test/output") {
        eprintln!("Failed to delete images: {}", err);
    }
    // let file_path = "./test/results/result_disruptor.csv";
    // let file_path = "./test/results/result_single_thread.csv";
    // let file_path: &str = "./test/results/result_multi_thread.csv";
    // let file_path: &str = "./test/results/result_x_beam.csv";
    // if !file_exists(&file_path) {
    //     let mut wtr = WriterBuilder::new()
    //         .from_path(&file_path)
    //         .expect("Failed to create csv file");
    //     if let Err(err) = wtr.write_record(&[&start_time.elapsed().as_secs_f64().to_string()]) {
    //         eprintln!("Failed to write to csv file: {}", err);
    //     }
    // } else {
    //     let mut file = OpenOptions::new()
    //         .write(true)
    //         .append(true)
    //         .open(&file_path)
    //         .expect("Failed to open csv file for append");
    //     if let Err(err) = writeln!(&mut file, "{}", start_time.elapsed().as_secs_f64()) {
    //         eprintln!("Failed to append to csv file: {}", err);
    //     }
    // }
}

fn delete_all_images(folder_path: &str) -> Result<(), std::io::Error> {
    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(extension) = path.extension() {
            if extension == "avif" {
                fs::remove_file(path)?;
            }
        }
    }
    Ok(())
}
fn file_exists(file_path: &str) -> bool {
    fs::metadata(file_path).is_ok()
}
