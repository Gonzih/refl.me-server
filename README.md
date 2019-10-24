[![Deploy](https://www.herokucdn.com/deploy/button.svg)](https://heroku.com/deploy)

Simple web server to be used with https://refl.me/ app.

Contains 2 endpoints:
* `/push`
* `/pop`

Just add `http://mydeploymenturl.com/pop` to REFL.ME app on your phone and you are good to go.

Pushing a message:
```
url -H 'Content-Type: application/json' -X POST -d '{"title":"Test!","message":"hello!"}' "http://mydeploymenturl.com/push"
```
