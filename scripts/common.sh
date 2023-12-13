while getopts ":c:d:r:p:" opt; do  
case $opt in
    c) CHAIN_ID=$OPTARG;;  
    d) DEV_ACCOUNT=$OPTARG;;  
    r) RPC_URL=$OPTARG;;  
    p) POC_CONTRACT_ADDRESS=$OPTARG;;  
    *) usage  
esac  
done  

# wasm_execute <EXECUTE_MSG> <AMOUNT>
wasm_execute() {

    OUTPUT="$(seda-chaind tx wasm execute $POC_CONTRACT_ADDRESS $1 --from $DEV_ACCOUNT --keyring-backend test --node $RPC_URL --gas-prices 0.025aseda --gas auto --gas-adjustment 1.3 -y --output json --chain-id $CHAIN_ID --amount "$2"seda)"
    echo $OUTPUT

    TXHASH=$(echo "$OUTPUT" | jq -r '.txhash')
    echo $TXHASH

    sleep 10
}

# sudo_call <SUDO_MSG>
sudo_call() {

    OUTPUT="$(seda-chaind tx wasm submit-proposal sudo-contract $POC_CONTRACT_ADDRESS $1 --from $DEV_ACCOUNT --keyring-backend test --node $RPC_URL --gas-prices 0.025aseda --gas auto --gas-adjustment 1.3 -y --output json --chain-id $CHAIN_ID --title "sudo-execute"  --summary "post_results" --deposit "$2"seda)"
    echo $OUTPUT

    TXHASH=$(echo "$OUTPUT" | jq -r '.txhash')
    echo $TXHASH

    sleep 10
}



# wasm_query <QUERY_MSG>
wasm_query() {

    OUTPUT="$(seda-chaind query wasm contract-state smart $POC_CONTRACT_ADDRESS $1 --node $RPC_URL --output json --chain-id $CHAIN_ID)"
    echo $OUTPUT

    sleep 10
}





# Function to generate a random array of strings as JSON
generate_random_array_json() {
    local array_size=$1
    local string_length=$2

    # Check if array size and string length are provided
    if [ -z "$array_size" ] || [ -z "$string_length" ]; then
        echo "Usage: generate_random_array_json <array_size> <string_length>"
        return 1
    fi

    # Generate random strings and populate the array
    local random_array=()
    for ((i = 0; i < array_size; i++)); do
        random_string=$(cat /dev/urandom | tr -dc 'a-zA-Z0-9' | head -c $string_length)
        random_array+=("$random_string")
    done

    # Convert the array to JSON format
    json_output='{"results":['
    for ((i = 0; i < ${#random_array[@]}; i++)); do
        json_output+="\"${random_array[i]}\""
        if [ $i -lt $(( ${#random_array[@]} - 1 )) ]; then
            json_output+=','
        fi
    done
    json_output+=']}'

    echo "$json_output"
}
