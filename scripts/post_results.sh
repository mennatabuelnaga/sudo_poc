#!/usr/bin/env bash

# usage: bash scripts/post_results.sh -c "sedachain" -d "seda12nmulx864e9ggymhf3tmavrmr6lse9l3qr4f4q" -r "http://127.0.0.1:26657" -p seda1qg5ega6dykkxc307y25pecuufrjkxkaggkkxh7nad0vhyhtuhw3suc9lqj


source scripts/common.sh


# Example usage: generate a random array of 5 strings, each with a length of 8
generated_array=$(generate_random_array_json 10 8)
# echo "Random Array: ${generated_array}"


# echo '{"post_results":'${generated_array}'}'
wasm_execute '{"post_results":'${generated_array}'}' 0
