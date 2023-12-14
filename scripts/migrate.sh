#!/usr/bin/env bash

# usage: bash scripts/migrate.sh -c "sedachain" -d "seda15krxaq358w02j55p3u7zqp0f3mupaxyt728d9q" -p seda1qg5ega6dykkxc307y25pecuufrjkxkaggkkxh7nad0vhyhtuhw3suc9lqj -r "http://127.0.0.1:26657" -i 4


source scripts/common.sh


migrate_call $POC_CONTRACT_ADDRESS $CODE_ID '{"v1_0_0_to_v2_0_0":{}}'