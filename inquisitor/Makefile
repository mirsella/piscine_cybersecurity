all:
	cargo build --release
	cp target/release/inquisitor .
	docker compose up -d --build

stop:
	docker compose stop

clean:
	docker compose down
