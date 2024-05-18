#!/bin/bash

HOUSE_NAME=MyHome

curl  --request POST \
    --url "http://localhost:5000/houses" \
    --header "Content-Type: application/json" \
    --data '{"name": "'$HOUSE_NAME'"}'