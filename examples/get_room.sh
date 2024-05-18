#!/bin/bash

HOUSE_ID=6
ROOM_ID=10

curl  --request GET \
    --url "http://localhost:5000/houses/$HOUSE_ID/rooms/$ROOM_ID" \
    --header "Content-Type: application/json"