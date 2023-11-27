# usage: bash scripts/sudo.sh -c "sedachain" -d "seda12nmulx864e9ggymhf3tmavrmr6lse9l3qr4f4q" -r "http://127.0.0.1:26657" -p seda12tc6cpv3e49z2me3xcm3064zkvrd7suqh3xaages9kq6adnnhjxq7ehgyk


# bash scripts/sudo.sh -c "my-chain" -d "wasm1kemm5uhf2rqzv8l7a3adj807c9kw66sauu2v77" -r "http://127.0.0.1:26657" -p "wasm14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9s0phg4d"

source scripts/common.sh


generated_array=$(generate_random_array_json 100000 8)
# echo "Random Array: ${generated_array}"


# echo '{"post_results":'${generated_array}'}'

sudo_call '{"post_results":'${generated_array}'}'
