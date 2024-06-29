CONF_PATH = aa

dev:
	RUST_BACKTRACE=1 cargo run -- --conf_path $(CONF_PATH)

test-rpc:
	cargo test test_rpc -- --nocapture
