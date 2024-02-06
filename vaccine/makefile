all: build

build:
	cargo build
	cp -v ./target/debug/vaccine .

tests: clean
	docker compose -f ./mysql-injection/docker-compose.yml up -d
	docker compose -f ./sqlite-injection/docker-compose.yml up -d

clean:
	docker compose -f ./mysql-injection/docker-compose.yml down
	docker compose -f ./sqlite-injection/docker-compose.yml down
