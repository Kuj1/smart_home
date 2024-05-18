#!/bin/bash

curl  --request GET \
    --url "http://localhost:5000/houses" \
    --header "Content-Type: application/json"