# Simple faucet

Contract deployment with one group:

```bash
export CONTRACT_ID=$(date +%s)-faucet.testnet

near account create-account sponsor-by-faucet-service $CONTRACT_ID autogenerate-new-keypair save-to-keychain network-config testnet create

near contract deploy $CONTRACT_ID use-file res/simple_faucet_contract.wasm with-init-call new json-args '{"transfer_amount": "10000000000000000000000", "approved_group": 0, "num_groups": 1, "start_block_height": 1}' prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' network-config testnet sign-with-keychain send
```

Contract call

```bash
export ACCOUNT_ID=$(date +%s)-test.testnet

near account create-account sponsor-by-faucet-service $ACCOUNT_ID autogenerate-new-keypair save-to-keychain network-config testnet create

# Views
near contract call-function as-read-only $CONTRACT_ID get_num_claims json-args {} network-config testnet now
near contract call-function as-read-only $CONTRACT_ID get_number_of_possible_claims json-args {} network-config testnet now
near contract call-function as-read-only $CONTRACT_ID get_current_block_height json-args {} network-config testnet now
near contract call-function as-read-only $CONTRACT_ID get_remaining_balance json-args {} network-config testnet now

near contract call-function as-read-only $CONTRACT_ID get_account_group json-args '{"account_id": "'$ACCOUNT_ID'"}' network-config testnet now
near contract call-function as-read-only $CONTRACT_ID can_claim json-args '{"account_id": "'$ACCOUNT_ID'"}' network-config testnet now

# Claim

near contract call-function as-transaction $CONTRACT_ID claim json-args {} prepaid-gas '30.0 Tgas' attached-deposit '0 NEAR' sign-as $ACCOUNT_ID network-config testnet sign-with-keychain send

# Verifying changes

near contract call-function as-read-only $CONTRACT_ID get_num_claims json-args {} network-config testnet now
near contract call-function as-read-only $CONTRACT_ID get_number_of_possible_claims json-args {} network-config testnet now
near contract call-function as-read-only $CONTRACT_ID get_current_block_height json-args {} network-config testnet now
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

near contract deploy $CONTRACT_ID use-file res/simple_faucet_contract.wasm with-init-call new json-args '{"transfer_amount": "10000000000000000000000", "approved_group": 1, "num_groups": 2, "start_block_height": 1}' prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' network-config testnet sign-with-keychain send
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
export CUR_BLOCK_HEIGHT=$(near contract call-function as-read-only $CONTRACT_ID get_current_block_height json-args {} network-config testnet now)
echo "Current block height: $CUR_BLOCK_HEIGHT"
# About 1 minute ahead
export START_BLOCK_HEIGHT=$(expr $CUR_BLOCK_HEIGHT + 60)
echo "Start block height (in about 1 minute): $START_BLOCK_HEIGHT"

export CONTRACT_ID=$(date +%s)-faucet.testnet

near account create-account sponsor-by-faucet-service $CONTRACT_ID autogenerate-new-keypair save-to-keychain network-config testnet create

sleep 5

near contract deploy $CONTRACT_ID use-file res/simple_faucet_contract.wasm with-init-call new json-args '{"transfer_amount": "10000000000000000000000", "approved_group": 0, "num_groups": 1, "start_block_height": '$START_BLOCK_HEIGHT'}' prepaid-gas '100.0 Tgas' attached-deposit '0 NEAR' network-config testnet sign-with-keychain send

sleep 5

near contract call-function as-read-only $CONTRACT_ID get_current_block_height json-args {} network-config testnet now
near contract call-function as-read-only $CONTRACT_ID get_start_block_height json-args {} network-config testnet now

# Can't claim because block height is too low
near contract call-function as-read-only $CONTRACT_ID can_claim json-args '{"account_id": "'$ACCOUNT_ID'"}' network-config testnet now

# Will be able to claim with a given block 
near contract call-function as-read-only $CONTRACT_ID can_claim json-args '{"account_id": "'$ACCOUNT_ID'", "block_height": '$START_BLOCK_HEIGHT'}' network-config testnet now

# Trying to claim (should fail with "Too early to claim")
near contract call-function as-transaction $CONTRACT_ID claim json-args {} prepaid-gas '30.0 Tgas' attached-deposit '0 NEAR' sign-as $ACCOUNT_ID network-config testnet sign-with-keychain send

# Waiting for the block height
echo "Waiting for 60 seconds the block height to reach $START_BLOCK_HEIGHT"
sleep 60

# Verifying we can claim
near contract call-function as-read-only $CONTRACT_ID get_current_block_height json-args {} network-config testnet now
near contract call-function as-read-only $CONTRACT_ID get_start_block_height json-args {} network-config testnet now
near contract call-function as-read-only $CONTRACT_ID can_claim json-args '{"account_id": "'$ACCOUNT_ID'"}' network-config testnet now

# Claiming
near contract call-function as-transaction $CONTRACT_ID claim json-args {} prepaid-gas '30.0 Tgas' attached-deposit '0 NEAR' sign-as $ACCOUNT_ID network-config testnet sign-with-keychain send

# Verifying changes
near contract call-function as-read-only $CONTRACT_ID get_num_claims json-args {} network-config testnet now
near contract call-function as-read-only $CONTRACT_ID can_claim json-args '{"account_id": "'$ACCOUNT_ID'"}' network-config testnet now
```
