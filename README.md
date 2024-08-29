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

