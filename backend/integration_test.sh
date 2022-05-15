#!/bin/sh

set -eu

cargo run -- add test

response=$(curl -v --data '{"username":"test","password":"test"}' -H "Content-Type: application/json" http://localhost:8080/api/login)
printf "login response: %s\n\n" "$response"
token=$(printf "%s\n" "$response" | jq -r '.bearerToken')
printf "bearer key: %s\n\n" "$token"

response=$(curl -v http://localhost:8080/api/images/ --oauth2-bearer $token -F "file=@pipe.jpg")
printf "image response: %s\n\n" "$response"
image_key=$(printf "%s\n" "$response" | jq -r '.key')
printf "image key: %s\n\n" "$image_key"

response=$(curl -v http://localhost:8080/api/albums/ -H "Content-Type: application/json" --oauth2-bearer $token --data "{\"title\":\"draft\",\"coverKey\":\"$image_key\",\"draft\":true, \"timeframe\": {\"from\": 0, \"to\": 10}, \"imageKeys\": [\"$image_key\"]}")
printf "album post response: %s\\nn" "$response"
album_key=$(printf "%s\n" "$response" | jq -r '.key')
printf "album key: %s\n\n" "$album_key"

curl -v http://localhost:8080/api/albums/ -H "Content-Type: application/json" --oauth2-bearer $token --data "{\"title\":\"non draft\",\"coverKey\":\"$image_key\", \"timeframe\": {\"from\": 0, \"to\": 10}, \"imageKeys\": [\"$image_key\"]}"

response=$(curl -v http://localhost:8080/api/albums/$album_key -H "Content-Type: application/json" --oauth2-bearer $token)
printf "album get response: %s\n\n" "$(printf "%s" "$response" | jq)"

response=$(curl -v http://localhost:8080/api/users/ -H "Content-Type: application/json" --oauth2-bearer $token)
printf "users get response: %s\n\n" "$(printf "%s" "$response" | jq)"

response=$(curl -v "http://localhost:8080/api/albums/?draft=true" -H "Content-Type: application/json" --oauth2-bearer $token)
printf "albums get response: %s\n\n" "$(printf "%s" "$response" | jq)"

