use std::{env, fs::File, io::{BufRead, BufReader}, path::Path, sync::Arc, time::Instant};
use pulsar::{Pulsar, TokioExecutor, producer::ProducerOptions };

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let addr = env::var("PULSAR_ADDRESS")
        .ok()
        .unwrap_or_else(||String::from("pulsar://127.0.0.1:6650"));
    log::info!("producer address: {}", &addr);

    let topic = String::from("non-persistent://public/default/raw");
    log::info!("topic: {}", &topic);

    let path = env::var("FILE_PATH").ok().unwrap_or_else(|| String::from("pride_mini.txt"));
    let file_path = Path::new(path.as_str());
    if !file_path.exists() {
        log::error!("File does not exist");
    }
    if !file_path.is_file() {
        log::error!("Specified path in not a file");
    }


    let builder = Pulsar::builder(addr, TokioExecutor);

    let pulsar: Pulsar<_> = builder.build().await?;

    let mut producer = pulsar
        .producer()
        .with_topic(topic)
        .with_name("text_producer")
        .with_options(ProducerOptions {
            ..Default::default()
        }).build().await?;


    let start = Instant::now();

    let file = File::open(file_path).expect("Could not open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let string_line = line.expect("Failed to read line");
        producer.send(string_line).await?;
    };
    log::info!("Time to read/send lines: {} m_sec", start.elapsed().as_millis());

    producer.close().await.expect("FAILED TO CLOSE CONNECTION");
    Ok(())
}
