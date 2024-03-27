worker.ssh:
	docker exec -it rust-queue-worker /bin/bash
worker.run:
	docker exec -it rust-queue-worker sh -c "cargo run --bin main"
event_dispatcher:
	docker exec -it rust-queue-worker sh -c "cargo run --bin event_dispatcher"
dispatch:
	docker exec -it rust-queue-worker sh -c "cargo run --bin dispatch_job"
test:
	docker exec -it rust-queue-worker sh -c "cargo test"