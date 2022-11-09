rm -rf ./assets/image_as_bytes.txt
cargo test --workspace

arg="(\"sns-1-image\")"
dfx canister call soulbound_token clear_asset --type idl "$arg"


file="./assets/image_as_bytes.txt"
while read -r line; do
    arg=$(echo "$line" | tr '[' '{')
    arg=$(echo "$arg" | tr ']' '}')
    arg=$(echo "$arg" | tr ',' ';')
    arg="( \"sns-1-image\", vec $arg )"
    dfx canister call soulbound_token append_asset --type idl "$arg"
done <$file