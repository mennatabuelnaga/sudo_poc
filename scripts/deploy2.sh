CHAIN_ID="my-chain"
TXFLAG="--chain-id $CHAIN_ID --gas-prices 0.025stake --gas auto --gas-adjustment 1.3"



wasmd init my-node --chain-id $CHAIN_ID
wasmd keys add main 
wasmd keys add validator
wasmd genesis add-genesis-account $(wasmd keys show main -a) 10000000000stake
wasmd genesis add-genesis-account $(wasmd keys show validator -a) 10000000000stake
wasmd genesis gentx validator 10000000000stake --chain-id $CHAIN_ID
wasmd genesis collect-gentxs
wasmd genesis validate-genesis
wasmd start