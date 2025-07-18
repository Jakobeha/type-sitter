set -e
SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
echo "*** CLI TESTING json"
cargo run -p type-sitter-cli "$SCRIPTPATH/../vendor/tree-sitter-json" -o "$SCRIPTPATH/test-generated"
echo "*** CLI TESTING rust"
cargo run -p type-sitter-cli "$SCRIPTPATH/../vendor/tree-sitter-rust" -o "$SCRIPTPATH/test-generated" --use-yak-sitter
echo "*** CLI TESTING json node_types"
cargo run -p type-sitter-cli "$SCRIPTPATH/../vendor/tree-sitter-json/src/node-types.json" -o "$SCRIPTPATH/test-generated/alt"
echo "*** CLI TESTING rust queries"
cargo run -p type-sitter-cli "$SCRIPTPATH/../vendor/tree-sitter-rust/queries" -o "$SCRIPTPATH/test-generated/alt"
echo "*** CLI TESTING python node_types"
cargo run -p type-sitter-cli "$SCRIPTPATH/../vendor/tree-sitter-python" -o "$SCRIPTPATH/test-generated"
echo "*** CLI PASSED"