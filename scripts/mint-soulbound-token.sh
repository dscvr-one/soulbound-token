#!/bin/bash

principal=$(dfx identity get-principal)
argument="( principal \"$principal\")"
echo "$argument"
dfx canister call soulbound_token mint --type idl "$argument"