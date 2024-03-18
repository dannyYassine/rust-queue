worker.ssh:
	docker exec -it rust-queue-worker /bin/bash
worker.run:
	docker exec -it rust-queue-worker sh -c "cargo run"
test:
	docker exec -it rust-queue-worker sh -c "cargo test"