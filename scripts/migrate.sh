#!/usr/bin/env bash

# usage: bash scripts/migrate.sh -c "sedachain" -d "seda15krxaq358w02j55p3u7zqp0f3mupaxyt728d9q" -p seda14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9snnh0yy -r "http://127.0.0.1:26657" -i 2


source scripts/common.sh


migrate_call $POC_CONTRACT_ADDRESS $CODE_ID '{"migrate":{}}'