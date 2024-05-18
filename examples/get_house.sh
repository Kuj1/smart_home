#!/bin/bash

HOUSE_ID=5

curl  --request GET \
    --url "http://localhost:5000/houses/$HOUSE_ID" \
    --header "Content-Type: application/json"