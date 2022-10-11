## Publish response body from vehicle-pos periodically

### Data API

Public Transport - Realtime Vehicle Positions v2
<https://opendata.transport.nsw.gov.au/dataset/public-transport-realtime-vehicle-positions-v2>

### How to run

```bash
cp .env.sample .env
vim .env  # set correct `VEHICLE_POS_API_KEY`

RUST_LOG=debug cargo run
```
