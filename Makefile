run:
	cargo run
test-push:
	curl -H 'Content-Type: application/json' -X POST -d '{"message":"hola!"}' "http://localhost:8080/max/push"
test-pop:
	curl "http://localhost:8080/max/pop"
