#!/usr/bin/env bash

export SUBSTREAMS_API_TOKEN=$(curl https://auth.streamingfast.io/v1/auth/issue -s --data-binary '{"api_key":"'$STREAMINGFAST_KEY'"}' | jq -r .token)
echo "Token set on in SUBSTREAMS_API_TOKEN"
