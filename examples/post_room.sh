#!/bin/bash

HOUSE_ID=6
ROOM_NAME=MyRoom

curl  --request POST \
    --url "http://localhost:5000/houses/$HOUSE_ID/rooms" \
    --header "Content-Type: application/json" \
    --data '{"name": "'$ROOM_NAME'"}'