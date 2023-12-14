#!/usr/bin/env bash

# usage: bash scripts/get_migration_msg.sh -c "sedachain" -d "seda15krxaq358w02j55p3u7zqp0f3mupaxyt728d9q" -r "http://127.0.0.1:26657"


source scripts/common.sh


smart_query '{"get_migration_msg":{}}'