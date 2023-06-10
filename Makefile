default: build

build:
	cargo build

run_migrations:
	diesel migration run