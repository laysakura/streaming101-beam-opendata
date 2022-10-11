## Consume protobuf message and ... (TBD)

### Update protobuf generated classes

```bash
protoc -I=../schema/vehicle-pos/ --java_out=src/main/java/ ../schema/vehicle-pos/gtfs-realtime_1007_extension.proto
```
