use std::env;
use pulsar::{Pulsar, TokioExecutor, producer::ProducerOptions, Error as PulsarError};

#[tokio::main]
async fn main() -> Result<(), PulsarError> {
    env_logger::init();
    let addr = env::var("PULSAR_ADDRESS")
        .ok()
        .unwrap_or_else(||String::from("pulsar://127.0.0.1:6650"));
    dbg!(&addr);

    let topic = env::var("PULSAR_TOPIC")
        .ok()
        .unwrap_or_else(||String::from("non-persistent://public/default/test"));

    dbg!(&topic);

    let mut builder = Pulsar::builder(addr, TokioExecutor);

    let pulsar: Pulsar<_> = builder.build().await?;

    let mut producer = pulsar
        .producer()
        .with_topic(topic)
        .with_name("text_producer")
        .with_options(ProducerOptions {
            ..Default::default()
        }).build() .await?;


    let mut counter = 0;
    loop {
        producer
            .send(format!("message {}", counter))
            .await?;

        log::info!("Sent messsage count: {}", &counter);
        counter += 1;
        tokio::time::sleep(std::time::Duration::from_secs(1));
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
