#!/bin/bash

dfx identity use default
principal=$(dfx identity get-principal)
argument="( principal \"$principal\")"
echo "$argument"
dfx canister call soulbound_token add_admin --type idl "$argument"