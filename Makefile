default: build

build:
	cargo build

run-migrations:
	diesel migration run

rerun-migrations:
	diesel migration redo