EXPORT = export RUSTPATH=$(PWD)

test:
	$(EXPORT) && cargo test -- --test-threads 1
	
checks:
	$(EXPORT) && cargo fmt
	$(EXPORT) && cargo clippy
	