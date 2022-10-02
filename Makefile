.PHONY: build
build:
	echo "Building..."
	cargo bootimage

.PHONY: run
run:
	echo "Running..."
	#qemu-system-x86_64 -drive format=raw,file=target/x86_64-lumi/debug/bootimage-lumi.bin
	cargo run

.PHONY: connect
connect:
	echo "Connecting..."
	sleep 2
	tools/vncviewer64-1.12.0.exe 127.0.0.1:5900

.PHONY: run_and_connect
run_and_connect:
	$(MAKE) -j2 run connect

.PHONY: dev
dev: build run_and_connect
