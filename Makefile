run:
	cargo run
test-push:
	curl -H 'Content-Type: application/json' -X POST -d '{"message":"hola!"}' "http://localhost:8000/push"
test-pop:
	curl "http://localhost:8000/pop"
