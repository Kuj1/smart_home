#!/bin/bash

HOUSE_ID=6
ROOM_ID=11
DEVICE_ID=18

DEVICE_NAME=NewDeviceRenamed

curl -v --request PATCH \
    --url "http://localhost:5000/houses/$HOUSE_ID/rooms/$ROOM_ID/devices/rename/$DEVICE_ID" \
    --header "Content-Type: application/json" \
    --data '{
        "name": "'$DEVICE_NAME'"
    }'