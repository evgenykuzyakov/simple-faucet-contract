# Simple faucet

Contract deployment with one group:

```bash
export CONTRACT_ID=$(date +%s)-faucet.testnet

near account create-account sponsor-by-faucet-service $CONTRACT_ID autogenerate-new-keypair save-to-keychain network-config testnet create

near contract deploy $CONTRACT_ID use-file res/simple_faucet_contract.wasm with-init-call new json-args '{"transfer_amount": "10000000000000000000000", "approved_group": 0, "num_groups": 1, "start_time_ms": 1}' prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' network-config testnet sign-with-keychain send
```

Contract call

```bash
export ACCOUNT_ID=$(date +%s)-test.testnet

near account create-account sponsor-by-faucet-service $ACCOUNT_ID autogenerate-new-keypair save-to-keychain network-config testnet create

# Views
near contract call-function as-read-only $CONTRACT_ID get_num_claims json-args {} network-config testnet now
near contract call-function as-read-only $CONTRACT_ID get_number_of_possible_claims json-args {} network-config testnet now
near contract call-function as-read-only $CONTRACT_ID get_current_time_ms json-args {} network-config testnet now
near contract call-function as-read-only $CONTRACT_ID get_remaining_balance json-args {} network-config testnet now

near contract call-function as-read-only $CONTRACT_ID get_account_group json-args '{"account_id": "'$ACCOUNT_ID'"}' network-config testnet now
near contract call-function as-read-only $CONTRACT_ID can_claim json-args '{"account_id": "'$ACCOUNT_ID'"}' network-config testnet now

# Claim

near contract call-function as-transaction $CONTRACT_ID claim json-args {} prepaid-gas '30.0 Tgas' attached-deposit '0 NEAR' sign-as $ACCOUNT_ID network-config testnet sign-with-keychain send

# Verifying changes

near contract call-function as-read-only $CONTRACT_ID get_num_claims json-args {} network-config testnet now
near contract call-function as-read-only $CONTRACT_ID get_number_of_possible_claims json-args {} network-config testnet now
near contract call-function as-read-only $CONTRACT_ID get_current_time_ms json-args {} network-config testnet now
near contract call-function as-read-only $CONTRACT_ID get_remaining_balance json-args {} network-config testnet now

near contract call-function as-read-only $CONTRACT_ID can_claim json-args '{"account_id": "'$ACCOUNT_ID'"}' network-config testnet now

# Try to claim again (should fail with "Already claimed")

near contract call-function as-transaction $CONTRACT_ID claim json-args {} prepaid-gas '30.0 Tgas' attached-deposit '0 NEAR' sign-as $ACCOUNT_ID network-config testnet sign-with-keychain send
```

## Multiple groups

Contract deployment with multiple groups:

```bash
export CONTRACT_ID=$(date +%s)-faucet.testnet

near account create-account sponsor-by-faucet-service $CONTRACT_ID autogenerate-new-keypair save-to-keychain network-config testnet create

near contract deploy $CONTRACT_ID use-file res/simple_faucet_contract.wasm with-init-call new json-args '{"transfer_amount": "10000000000000000000000", "approved_group": 0, "num_groups": 3, "start_time_ms": 1}' prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' network-config testnet sign-with-keychain send
```

Contract call

```bash
export ACCOUNT_ID2=$(date +%s)-test.testnet

near account create-account sponsor-by-faucet-service $ACCOUNT_ID2 autogenerate-new-keypair save-to-keychain network-config testnet create

# Views
near contract call-function as-read-only $CONTRACT_ID get_approved_group json-args {} network-config testnet now
near contract call-function as-read-only $CONTRACT_ID get_num_groups json-args {} network-config testnet now
near contract call-function as-read-only $CONTRACT_ID get_num_claims json-args {} network-config testnet now

near contract call-function as-read-only $CONTRACT_ID get_account_group json-args '{"account_id": "'$ACCOUNT_ID'"}' network-config testnet now
near contract call-function as-read-only $CONTRACT_ID can_claim json-args '{"account_id": "'$ACCOUNT_ID'"}' network-config testnet now
near contract call-function as-read-only $CONTRACT_ID get_account_group json-args '{"account_id": "'$ACCOUNT_ID2'"}' network-config testnet now
near contract call-function as-read-only $CONTRACT_ID can_claim json-args '{"account_id": "'$ACCOUNT_ID2'"}' network-config testnet now

# Claim (may fail due to the assigned groups)

near contract call-function as-transaction $CONTRACT_ID claim json-args {} prepaid-gas '30.0 Tgas' attached-deposit '0 NEAR' sign-as $ACCOUNT_ID network-config testnet sign-with-keychain send
near contract call-function as-transaction $CONTRACT_ID claim json-args {} prepaid-gas '30.0 Tgas' attached-deposit '0 NEAR' sign-as $ACCOUNT_ID2 network-config testnet sign-with-keychain send

# Verifying changes

near contract call-function as-read-only $CONTRACT_ID get_num_claims json-args {} network-config testnet now

near contract call-function as-read-only $CONTRACT_ID can_claim json-args '{"account_id": "'$ACCOUNT_ID'"}' network-config testnet now
near contract call-function as-read-only $CONTRACT_ID can_claim json-args '{"account_id": "'$ACCOUNT_ID2'"}' network-config testnet now
```

# After block height

Contract deployment with a future block height and immediate testing:

```bash
export CUR_TIME_MS=$(near contract call-function as-read-only $CONTRACT_ID get_current_time_ms json-args {} network-config testnet now)
echo "Current time ms (from chain): $CUR_TIME_MS"
# About 1 minute ahead
export START_TIME_MS=$(expr $CUR_TIME_MS + 60000)
echo "Start time ms (in 1 minute): $START_TIME_MS"

export CONTRACT_ID=$(date +%s)-faucet.testnet

near account create-account sponsor-by-faucet-service $CONTRACT_ID autogenerate-new-keypair save-to-keychain network-config testnet create

sleep 5

near contract deploy $CONTRACT_ID use-file res/simple_faucet_contract.wasm with-init-call new json-args '{"transfer_amount": "10000000000000000000000", "approved_group": 0, "num_groups": 1, "start_time_ms": '$START_TIME_MS'}' prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' network-config testnet sign-with-keychain send

sleep 5

near contract call-function as-read-only $CONTRACT_ID get_current_time_ms json-args {} network-config testnet now
near contract call-function as-read-only $CONTRACT_ID get_start_time_ms json-args {} network-config testnet now

# Can't claim because block height is too low
near contract call-function as-read-only $CONTRACT_ID can_claim json-args '{"account_id": "'$ACCOUNT_ID'"}' network-config testnet now

# Will be able to claim with a given block 
near contract call-function as-read-only $CONTRACT_ID can_claim json-args '{"account_id": "'$ACCOUNT_ID'", "block_height": '$START_TIME_MS'}' network-config testnet now

# Trying to claim (should fail with "Too early to claim")
near contract call-function as-transaction $CONTRACT_ID claim json-args {} prepaid-gas '30.0 Tgas' attached-deposit '0 NEAR' sign-as $ACCOUNT_ID network-config testnet sign-with-keychain send

# Waiting for the block height
echo "Waiting for 60 seconds"
sleep 60

# Verifying we can claim
near contract call-function as-read-only $CONTRACT_ID get_current_time_ms json-args {} network-config testnet now
near contract call-function as-read-only $CONTRACT_ID get_start_time_ms json-args {} network-config testnet now
near contract call-function as-read-only $CONTRACT_ID can_claim json-args '{"account_id": "'$ACCOUNT_ID'"}' network-config testnet now

# Claiming
near contract call-function as-transaction $CONTRACT_ID claim json-args {} prepaid-gas '30.0 Tgas' attached-deposit '0 NEAR' sign-as $ACCOUNT_ID network-config testnet sign-with-keychain send

# Verifying changes
near contract call-function as-read-only $CONTRACT_ID get_num_claims json-args {} network-config testnet now
near contract call-function as-read-only $CONTRACT_ID can_claim json-args '{"account_id": "'$ACCOUNT_ID'"}' network-config testnet now
```

## Multiple groups on different shards

Contract deployment with multiple groups:

```bash
export ROOT_CONTRACT_ID="$(date +%s)-faucet.testnet"
near account create-account sponsor-by-faucet-service $ROOT_CONTRACT_ID autogenerate-new-keypair save-to-keychain network-config testnet create

export CONTRACT_ID_0="0.$ROOT_CONTRACT_ID"
near account create-account fund-myself $CONTRACT_ID_0 '1.9 NEAR' autogenerate-new-keypair save-to-keychain sign-as $ROOT_CONTRACT_ID network-config testnet sign-with-keychain send

export CONTRACT_ID_1="b.$ROOT_CONTRACT_ID"
near account create-account fund-myself $CONTRACT_ID_1 '1.9 NEAR' autogenerate-new-keypair save-to-keychain sign-as $ROOT_CONTRACT_ID network-config testnet sign-with-keychain send

export CONTRACT_ID_2="h.$ROOT_CONTRACT_ID"
near account create-account fund-myself $CONTRACT_ID_2 '1.9 NEAR' autogenerate-new-keypair save-to-keychain sign-as $ROOT_CONTRACT_ID network-config testnet sign-with-keychain send

export CONTRACT_ID_3="m.$ROOT_CONTRACT_ID"
near account create-account fund-myself $CONTRACT_ID_3 '1.9 NEAR' autogenerate-new-keypair save-to-keychain sign-as $ROOT_CONTRACT_ID network-config testnet sign-with-keychain send

export CONTRACT_ID_4="z.$ROOT_CONTRACT_ID"
near account create-account fund-myself $CONTRACT_ID_4 '1.9 NEAR' autogenerate-new-keypair save-to-keychain sign-as $ROOT_CONTRACT_ID network-config testnet sign-with-keychain send

# Start in 10 minutes
export CURRENT_TIME_S=$(date +%s)
export START_TIME_MS=$(expr $CURRENT_TIME_S \* 1000 + 600000)
near contract deploy $CONTRACT_ID_0 use-file res/simple_faucet_contract.wasm with-init-call new json-args '{"transfer_amount": "10000000000000000000000", "approved_group": 0, "num_groups": 5, "start_time_ms": '$START_TIME_MS'}' prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' network-config testnet sign-with-keychain send
near contract deploy $CONTRACT_ID_1 use-file res/simple_faucet_contract.wasm with-init-call new json-args '{"transfer_amount": "10000000000000000000000", "approved_group": 1, "num_groups": 5, "start_time_ms": '$START_TIME_MS'}' prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' network-config testnet sign-with-keychain send
near contract deploy $CONTRACT_ID_2 use-file res/simple_faucet_contract.wasm with-init-call new json-args '{"transfer_amount": "10000000000000000000000", "approved_group": 2, "num_groups": 5, "start_time_ms": '$START_TIME_MS'}' prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' network-config testnet sign-with-keychain send
near contract deploy $CONTRACT_ID_3 use-file res/simple_faucet_contract.wasm with-init-call new json-args '{"transfer_amount": "10000000000000000000000", "approved_group": 3, "num_groups": 5, "start_time_ms": '$START_TIME_MS'}' prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' network-config testnet sign-with-keychain send
near contract deploy $CONTRACT_ID_4 use-file res/simple_faucet_contract.wasm with-init-call new json-args '{"transfer_amount": "10000000000000000000000", "approved_group": 4, "num_groups": 5, "start_time_ms": '$START_TIME_MS'}' prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' network-config testnet sign-with-keychain send
```
