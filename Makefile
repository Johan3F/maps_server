DEV_DB=maps
DEV_DB_USER=username
DEV_DB_PASS=password

default: build

build:
	cargo build

.phony: run_migrations

run_migrations:
	diesel migration run --database-url=postgres://${DEV_DB_USER}:${DEV_DB_PASS}@localhost/${DEV_DB}
