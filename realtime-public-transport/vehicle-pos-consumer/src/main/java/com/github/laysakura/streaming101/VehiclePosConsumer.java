package com.github.laysakura.streaming101;

import java.time.Instant;
import java.time.ZoneId;
import java.time.ZonedDateTime;

import org.apache.beam.sdk.Pipeline;
import org.apache.beam.sdk.io.TextIO;
import org.apache.beam.sdk.io.kafka.KafkaIO;
import org.apache.beam.sdk.options.PipelineOptions;
import org.apache.beam.sdk.options.PipelineOptionsFactory;
import org.apache.beam.sdk.transforms.DoFn;
import org.apache.beam.sdk.transforms.MapElements;
import org.apache.beam.sdk.transforms.ParDo;
import org.apache.beam.sdk.transforms.SimpleFunction;
import org.apache.beam.sdk.transforms.Values;
import org.apache.beam.sdk.values.KV;
import org.apache.beam.sdk.values.PCollection;
import org.apache.kafka.common.serialization.ByteArrayDeserializer;

import com.google.protobuf.InvalidProtocolBufferException;
import com.google.transit.realtime.GtfsRealtime1007Extension.FeedHeader;
import com.google.transit.realtime.GtfsRealtime1007Extension.FeedMessage;

public class VehiclePosConsumer {

  public interface VehiclePosConsumerOptions extends PipelineOptions {
  }

  static void runWordCount(VehiclePosConsumerOptions options) {
    Pipeline p = Pipeline.create(options);

    PCollection<KV<byte[], byte[]>> kafkaRecord = p.apply("read from Kafka", KafkaIO.readBytes()
        .withBootstrapServers("localhost:9092")
        .withTopic("vehicle-pos")
        .withKeyDeserializer(ByteArrayDeserializer.class)
        .withValueDeserializer(ByteArrayDeserializer.class)

        // We're writing to a file, which does not support unbounded data sources. This
        // line makes it bounded to
        // the first 5 records.
        // In reality, we would likely be writing to a data source that supports
        // unbounded data, such as BigQuery.
        .withMaxNumRecords(5)

        .withoutMetadata() // KafkaRecord -> KV
    );

    PCollection<FeedMessage> feedMessage = kafkaRecord
        .apply("extract value from KV", Values.<byte[]>create())
        .apply("deserialize to FeedMessage",
            ParDo.of(new DoFn<byte[], FeedMessage>() {
              @ProcessElement
              public void processElement(@Element byte[] serialized,
                  OutputReceiver<FeedMessage> outFeedMessage) throws InvalidProtocolBufferException {

                FeedMessage feedMessage = FeedMessage.parseFrom(serialized);
                outFeedMessage.output(feedMessage);
              }
            }));

    PCollection<Long> unixtime = feedMessage.apply("extract unixtime from FeedHeader",
        MapElements.via(new SimpleFunction<FeedMessage, Long>() {
          @Override
          public Long apply(FeedMessage feedMessage) {
            FeedHeader feedHeader = feedMessage.getHeader();
            return feedHeader.getTimestamp();
          }
        }));

    unixtime
        .apply("FormatResults", MapElements.via(new SimpleFunction<Long, String>() {
          @Override
          public String apply(Long unixtime) {
            Instant instant = Instant.ofEpochSecond(unixtime);
            ZonedDateTime zonedDateTime = ZonedDateTime.ofInstant(instant, ZoneId.of("Asia/Tokyo"));
            return zonedDateTime.toString();
          }
        }))
        .apply(TextIO.write().to("unixtime"));

    p.run().waitUntilFinish();
  }

  public static void main(String[] args) {
    VehiclePosConsumerOptions options = PipelineOptionsFactory.fromArgs(args).withValidation()
        .as(VehiclePosConsumerOptions.class);

    runWordCount(options);
  }
}
