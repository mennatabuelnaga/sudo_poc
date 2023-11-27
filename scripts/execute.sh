#!/usr/bin/env bash

# usage: bash scripts/execute.sh -c "sedachain" -d "seda12nmulx864e9ggymhf3tmavrmr6lse9l3qr4f4q" -r "http://127.0.0.1:26657" -p seda12tc6cpv3e49z2me3xcm3064zkvrd7suqh3xaages9kq6adnnhjxq7ehgyk

source scripts/common.sh

wasm_execute '{"post_results":{"results":["hello"]}}' 0
