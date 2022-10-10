## Architecture

```text
Input data: Public Transport - Realtime Vehicle Positions v2
<https://opendata.transport.nsw.gov.au/dataset/public-transport-realtime-vehicle-positions-v2>
  |
  v
vehicle-pos-producer (Rust app)
  |
  | Protocol Buffers (gtfs-realtime_1007_extension.proto)
  v
Kafka (topic: vehicle-pos)
  |
  v
vehicle-pos-consumer (Beam w/ Flink)
  |
  v
Kafka (topic: vehicle-result)
```
