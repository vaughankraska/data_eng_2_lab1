use std::{env, fs::File, io::{BufRead, BufReader, Read}, path::Path};
use pulsar::{Pulsar, TokioExecutor, producer::ProducerOptions, Error as PulsarError};

#[tokio::main]
async fn main() -> Result<(), PulsarError> {
    env_logger::init();
    let addr = env::var("PULSAR_ADDRESS")
        .ok()
        .unwrap_or_else(||String::from("pulsar://127.0.0.1:6650"));
    log::info!("producer address: {}", &addr);

    let topic = env::var("PULSAR_TOPIC")
        .ok()
        .unwrap_or_else(||String::from("non-persistent://public/default/test"));
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

    let file = File::open(file_path).expect("Could not open file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let string_line = line.expect("Failed to read line");
        println!("line:{}", string_line);
    };


    let mut counter: usize = 0;

    loop {
        producer
            .send(format!("message {}", counter))
            .await?;

        log::info!("Sent messsage count: {}", &counter);
        counter += 1;
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        if counter > 10 {
            producer
                .close()
                .await
                .expect("FAILED TO CLOSE CONNECTION");
            break;
        }
    }


    Ok(())
}
