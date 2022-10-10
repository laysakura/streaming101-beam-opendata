use std::time::Duration;

use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let produce_future = producer.send(
        FutureRecord::to("vehicle-pos")
            .key("some key")
            .payload("some payload"),
        Duration::from_secs(0),
    );
    match produce_future.await {
        Ok(delivery) => println!("Sent: {:?}", delivery),
        Err((e, _)) => println!("Error: {:?}", e),
    }

    Ok(())
}
