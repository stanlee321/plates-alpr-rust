# Plates server


## Installation

```bash
curl -X POST -H "Content-Type: application/json" -d '{"field1":"value1", "field2":"value2"}' https://a53b-2800-cd0-7e01-a400-943d-8b17-e8e8-a19f.ngrok-free.app/webhook

```



```bash
curl -X POST -H "Content-Type: application/json" -d '{"field1":"value1", "field2":"value2"}' http://127.0.0.1:8080/webhook

```


## Build

```bash
docker build -t stanlee321/kafka-arm64:latest ./kafka
docker buildx build --platform linux/amd64 stanlee321/kafka:latest .
docker buildx build --platform linux/amd64  -t stanlee321/kafka --load -f Dockerfile .

```

## Push

```bash

docker push stanlee321/kafka:latest

```

## Test hook

kafka-console-consumer.sh --bootstrap-server localhost:9092 --topic plates --from-beginning