# usage: bash scripts/sudo.sh -c "sedachain" -d "seda1hlqzp7ngss290yw6nlrdujp759ejrhlrkgwccq" -r "http://127.0.0.1:26657" -p seda14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9snnh0yy



source scripts/common.sh


generated_array=$(generate_random_array_json 1000000 8)


sudo_call '{"infinite":'{}'}' 2
