#!/bin/bash

HOUSE_ID=5

curl  --request DELETE \
    --url "http://localhost:5000/houses/$HOUSE_ID" \
    --header "Content-Type: application/json"