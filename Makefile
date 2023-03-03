all: env start

env:
	cp .default.env .env

start:
	docker compose -f .container/docker-compose.yml up -d

stop:
	docker compose -f .container/docker-compose.yml down

db-setup:
	@bash .container/scripts/db-setup.sh

db: db-setup