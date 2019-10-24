run:
	cargo run
test-push:
	curl -H 'Content-Type: application/json' -X POST -d '{"message":"hola!", "title":"TITLE", "image":"https://i.kym-cdn.com/entries/icons/original/000/021/807/4d7.png"}' "http://localhost:8080/max/push"
test-pop:
	curl "http://localhost:8080/max/pop"
