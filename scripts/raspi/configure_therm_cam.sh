#!/bin/bash

# Check internet

if nc -zw1 google.com 443; then
    echo "There is internet"
fi
