db:
	docker run --rm -d --name postgres -p 5432:5432 \
  -e POSTGRES_DB=vinted-rs \
  -e POSTGRES_USER=postgres \
  -e POSTGRES_PASSWORD=postgres \
  postgres:latest


diesel:
	DATABASE_URL=postgres://postgres:postgres@localhost/vinted-rs diesel migration run
stop:
	docker kill postgres

clippy:
	cargo clippy --all-features