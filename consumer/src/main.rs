use std::env;
use pulsar::{
    message::{proto::command_subscribe::SubType, Payload},
    Consumer, Pulsar, TokioExecutor, Error as PulsarError,
};
use futures::TryStreamExt;


#[tokio::main]
async fn main() -> Result<(), PulsarError>{
    env_logger::init();

    let addr = env::var("PULSAR_ADDRESS")
        .ok()
        .unwrap_or_else(||String::from("pulsar://127.0.0.1:6650"));

    let topic = env::var("PULSAR_TOPIC")
        .ok()
        .unwrap_or_else(||String::from("non-persistent://public/default/test"));

    let pulsar: Pulsar<_> = Pulsar::builder(addr, TokioExecutor).build().await?;

    let mut consumer: Consumer<String, _> = pulsar
        .consumer()
        .with_topic(topic)
        .with_consumer_name("text_consumer")
        .with_options(pulsar::ConsumerOptions::default())
        .with_subscription_type(SubType::Exclusive)
        .build()
        .await?;

    let mut counter: usize = 0;

    while let Some(msg) = consumer.try_next().await? {
        consumer.ack(&msg).await?;
        log::info!("id: {:?}", msg.message_id());

        println!("string message: {:?}", msg.deserialize().expect("could not deserialize message"));

        log::info!("Processed {} messages", counter);
        counter += 1;
    };



    println!("EXITING CONSUMER");
    Ok(())
}
