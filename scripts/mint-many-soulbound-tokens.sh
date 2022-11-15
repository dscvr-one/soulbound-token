#!/bin/bash
dfx identity new --disable-encryption id1
dfx identity new --disable-encryption id2
dfx identity new --disable-encryption id3

dfx identity use id1
principal1=$(dfx identity get-principal)

dfx identity use id2
principal2=$(dfx identity get-principal)

dfx identity use id3
principal3=$(dfx identity get-principal)

argument="( vec { principal \"$principal1\"; principal \"$principal2\"; principal \"$principal3\" } )"
echo "$argument"
dfx identity use default
dfx canister call soulbound_token mint_many --type idl "$argument"
dfx canister call soulbound_token mint_many --type idl "$argument"