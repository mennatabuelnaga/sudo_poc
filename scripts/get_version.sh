#!/usr/bin/env bash

# usage: bash scripts/get_version.sh -c "sedachain" -d "seda1gs52z88gmek3ex73urxnf3p8jflywkd4e5ky2w" -r "http://127.0.0.1:26657" -p seda1qg5ega6dykkxc307y25pecuufrjkxkaggkkxh7nad0vhyhtuhw3suc9lqj


source scripts/common.sh


smart_query '{"get_version":{}}'