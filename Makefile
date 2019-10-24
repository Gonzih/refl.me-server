run:
	cargo run
test-push:
	curl -H 'Content-Type: application/json' -X POST -d '{"message":"hola!"}' "http://localhost:8000/max/push"
test-pop:
	curl "http://localhost:8000/max/pop"
