#!/bin/bash



near login
echo "[?] Near Network >>>"
read NETWORK



echo "--------------------- NEAR MASTER ACCOUNT DEPLOYMENT ---------------------"
echo "[?] Master Account ID - Logged In Account; With .testnet >>>"
read OWNER_ID # NOTE - the account id to (re)deploy the contract on which is the owner or the signer of the contract
NEAR_ENV=$NETWORK near deploy --wasmFile out/market.wasm --accountId $OWNER_ID



# NOET - the NEAR Runtime looks at your current code as well as your contract's data, which is serialized and saved on-disk. When it executes the code, it tries to match these up. If you change the code but the data stays the same, it can't figure out how to do this
# NOET - when our contract is executed, the NEAR Runtime reads the serialized state from disk and attempts to load it using current contract code by calling any init method inside the contract, when our code changes but the serialized state stays the same, it can't figure out how to do this thus it'll show us `Cannot deserialize the contract state`.
# NTOE - to avoid above note error we can redeploy the contract on a new sub account or delete the old one and redeploy the contract


echo "--------------------- NEAR SUB MASTER ACCOUNT DEPLOYMENT ---------------------"
echo "[?] Sub Master Account ID >>>"
read SUB_MASTER_CONTRACT_ID
if [ -z "$SUB_MASTER_CONTRACT_ID" ]
then
    echo "[?] No Sub Master Account Entered!"
else
    echo "[...] Deploying on Sub Master Account"
    near create-account $SUB_MASTER_CONTRACT_ID.$OWNER_ID --masterAccount $OWNER_ID --initialBalance 25
    NEAR_ENV=$NETWORK near deploy --wasmFile out/market.wasm --accountId $SUB_MASTER_CONTRACT_ID.$OWNER_ID
fi