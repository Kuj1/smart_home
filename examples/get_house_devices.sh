#!/bin/bash

HOUSE_ID=6

curl  --request GET \
    --url "http://localhost:5000/houses/$HOUSE_ID/devices" \
    --header "Content-Type: application/json"