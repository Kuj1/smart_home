#!/bin/bash

HOUSE_ID=6
ROOM_ID=11

DEVICE_VENDOR_ID=12FSA444
DEVICE_NAME=MyDevice
DEVICE_IS_ON=false
DEVICE_VOLTAGE=220
DEVICE_POWER=2A

curl  --request POST \
    --url "http://localhost:5000/houses/$HOUSE_ID/rooms/$ROOM_ID/devices" \
    --header "Content-Type: application/json" \
    --data '{
        "name": "'$DEVICE_NAME'",
        "vendor_id": "'$DEVICE_VENDOR_ID'",
        "is_on": '$DEVICE_IS_ON',
        "voltage": "'$DEVICE_VOLTAGE'",
        "power": "'$DEVICE_POWER'"
    }'