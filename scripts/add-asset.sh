rm -rf ./assets/image_as_bytes.txt
cargo test --workspace

dfx canister call soulbound_token clear_asset

file="./assets/image_as_bytes.txt"
while read -r line; do
    arg=$(echo "$line" | tr '[' '{')
    arg=$(echo "$arg" | tr ']' '}')
    arg=$(echo "$arg" | tr ',' ';')
    arg="( vec $arg )"
    dfx canister call soulbound_token append_asset --type idl "$arg"
done <$file