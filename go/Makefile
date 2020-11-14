build:
	go build -o ssm-get-parameter
build-alpine:
	docker run --rm -it -w /go/src/app -v "$(pwd)":/go/src/app golang:1.15-alpine go build -o ssm-get-parameter-alpine
