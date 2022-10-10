## Architecture

```text
Input data: Public Transport - Realtime Vehicle Positions v2
<https://opendata.transport.nsw.gov.au/dataset/public-transport-realtime-vehicle-positions-v2>
  |
  v
Kafka (topic: vehicle-pos)
  |
  v
Beam (Flink Runner)
  |
  v
Kafka (topic: xxx)
```
