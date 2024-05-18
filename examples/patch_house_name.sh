#!/bin/bash

HOUSE_ID=6

HOUSE_NAME=NewHouseRenamed

curl -v --request PATCH \
    --url "http://localhost:5000/houses/rename/$HOUSE_ID" \
    --header "Content-Type: application/json" \
    --data '{
        "name": "'$HOUSE_NAME'"
    }'