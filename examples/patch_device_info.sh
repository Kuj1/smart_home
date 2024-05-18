#!/bin/bash

HOUSE_ID=6
ROOM_ID=11
DEVICE_ID=18

DEVICE_IS_ON=true
DEVICE_VOLTAGE=231
DEVICE_POWER=3A

curl -v --request PATCH \
    --url "http://localhost:5000/houses/$HOUSE_ID/rooms/$ROOM_ID/devices/$DEVICE_ID" \
    --header "Content-Type: application/json" \
    --data '{
        "is_on": '$DEVICE_IS_ON',
        "voltage": "'$DEVICE_VOLTAGE'",
        "power": "'$DEVICE_POWER'"
    }'