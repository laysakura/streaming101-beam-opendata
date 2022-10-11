package com.github.laysakura.streaming101;

import org.apache.beam.sdk.Pipeline;
import org.apache.beam.sdk.io.kafka.KafkaIO;
import org.apache.beam.sdk.io.kafka.KafkaRecord;
import org.apache.beam.sdk.options.PipelineOptions;
import org.apache.beam.sdk.options.PipelineOptionsFactory;
import org.apache.beam.sdk.values.PCollection;
import org.apache.kafka.common.serialization.ByteArrayDeserializer;

public class VehiclePosConsumer {

  public interface VehiclePosConsumerOptions extends PipelineOptions {
  }

  static void runWordCount(VehiclePosConsumerOptions options) {
    Pipeline p = Pipeline.create(options);

    PCollection<KafkaRecord<byte[], byte[]>> kafka_record = p.apply(KafkaIO.readBytes()
        .withBootstrapServers("localhost:9092")
        .withTopic("vehicle-pos")
        .withKeyDeserializer(ByteArrayDeserializer.class)
        .withValueDeserializer(ByteArrayDeserializer.class));

    p.run().waitUntilFinish();
  }

  public static void main(String[] args) {
    VehiclePosConsumerOptions options = PipelineOptionsFactory.fromArgs(args).withValidation()
        .as(VehiclePosConsumerOptions.class);

    runWordCount(options);
  }
}
