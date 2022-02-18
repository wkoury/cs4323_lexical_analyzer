release:
	cargo build --release

run: release
	@echo "Running on Dr. Kim's test program..."
	./target/release/scanner kim_example.ssc

debug:
	cargo build
	@echo "Opening lldb..."
	lldb ./target/debug/scanner kim_example.ssc

watch:
	cargo watch -x fmt -x build -x test

test:
	cargo test

clean:
	cargo clean

loc: clean
	cloc .

flamegraph:
	sudo cargo flamegraph --dev -- kim_example.ssc

