IMAGE_NAME="stanlee321/plates_webhook"
IMAGE_TAG="latest"
run:
	cargo run --release

build:
	docker buildx build --platform linux/amd64 --memory=4g --memory-swap=4g -t $(IMAGE_NAME):$(IMAGE_TAG) --load -f Dockerfile .

