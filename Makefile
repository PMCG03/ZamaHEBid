performance_test:
	cargo test --release --test performance_Test -- --nocapture --test-threads=1

integration_test:
	cargo test --release --test integration_Test -- --nocapture --test-threads=1

unit_test:
	cargo test --release --test unit_Test -- --nocapture --test-threads=1

all_tests:
	cargo test --release -- --nocapture --test-threads=1

build:
	cargo build --release

run:
	cargo run --release
