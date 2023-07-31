EXPORT = export RUSTPATH=$(PWD)

test:
	$(EXPORT) && cargo test -- --test-threads 1 --nocapture      
checks:
	$(EXPORT) && cargo fmt
	$(EXPORT) && cargo clippy