#!/bin/bash



near login
echo "[?] Near Network >>>"
read NETWORK
echo "[?] Logged In Account To Deploy The Contract On; With .testnet >>>"
read OWNER_ID # NOTE - the account id to (re)deploy the contract on which is the owner and the signer of the contract
NEAR_ENV=$NETWORK near deploy --wasmFile out/event.wasm --accountId $OWNER_ID