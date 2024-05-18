#!/bin/bash

HOUSE_ID=6
ROOM_ID=11

ROOM_NAME=NewRoomRenamed

curl -v --request PATCH \
    --url "http://localhost:5000/houses/$HOUSE_ID/rooms/rename/$ROOM_ID" \
    --header "Content-Type: application/json" \
    --data '{
        "name": "'$ROOM_NAME'"
    }'