export RUST_LOG=debug

build:
	cargo build

run:
	docker-compose up -d
	cargo run