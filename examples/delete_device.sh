#!/bin/bash

HOUSE_ID=6
ROOM_ID=11
DEVICE_ID=10

curl  --request DELETE \
    --url "http://localhost:5000/houses/$HOUSE_ID/rooms/$ROOM_ID/devices/$DEVICE_ID" \
    --header "Content-Type: application/json"