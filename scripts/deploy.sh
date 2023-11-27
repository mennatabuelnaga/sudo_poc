#!/usr/bin/env bash

# usage: bash scripts/deploy.sh -c "sedachain" -d "seda12nmulx864e9ggymhf3tmavrmr6lse9l3qr4f4q" -r "http://127.0.0.1:26657"
# bash scripts/deploy.sh -c "my-chain" -d wasm1p0uj08ayglqlk8g790lsjdkj8p463ssps96r69 -r "http://127.0.0.1:26657" 

source scripts/common.sh

# store_contract CONTRACT_NAME
store_contract(){

    OUTPUT="$(wasmd tx wasm store "./artifacts/$1.wasm" --node $RPC_URL --from $DEV_ACCOUNT  --keyring-backend test --gas-prices 0.1stake --gas auto --gas-adjustment 1.6 -y --output json --chain-id $CHAIN_ID)"
    echo $OUTPUT

    TXHASH=$(echo $OUTPUT | jq -r '.txhash')
    echo $TXHASH

    sleep 10

    OUTPUT=$(wasmd query tx $TXHASH --node $RPC_URL --output json)
    echo $OUTPUT

    CODE_ID=$(echo "$OUTPUT" | jq -r '.logs[].events[] | select(.type=="store_code") | .attributes[] | select(.key=="code_id") | .value')
  
}

# instantiate_contract CODE_ID INSTANTIATE_MSG LABEL
instantiate_contract() {

    OUTPUT=$(wasmd tx wasm instantiate $1 $2 --no-admin --from $DEV_ACCOUNT  --keyring-backend test --node $RPC_URL --label "$3$1" --gas-prices 0.025stake --gas auto --gas-adjustment 2 -y --output json --chain-id $CHAIN_ID --broadcast-mode=block)
    echo $OUTPUT

    TXHASH=$(echo "$OUTPUT" | jq -r '.txhash')

    sleep 10


    OUTPUT="$(wasmd query tx $TXHASH --node $RPC_URL --output json)"
    echo $OUTPUT

    CONTRACT_ADDRESS=$(echo "$OUTPUT" | jq -r '.logs[].events[] | select(.type=="instantiate") | .attributes[] | select(.key=="_contract_address") | .value')


}


# Upload PoC contract
store_contract "poc_contract"
POC_CODE_ID=$CODE_ID

# Instantiate PoC
LABEL=poc
INSTANTIATE_MSG='{}'
instantiate_contract $POC_CODE_ID $INSTANTIATE_MSG $LABEL
POC_CONTRACT_ADDRESS=$CONTRACT_ADDRESS



echo "------------------------------------------"
echo "POC_CODE_ID:" $POC_CODE_ID

echo "------------------------------------------"
echo "POC_CONTRACT_ADDRESS: " $POC_CONTRACT_ADDRESS

echo "------------------------------------------"



