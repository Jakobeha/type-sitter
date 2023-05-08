set -e
SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
echo "*** CLI TESTING json"
cargo run -p type-sitter-cli "$SCRIPTPATH/../vendor/tree-sitter-json/src/node-types.json" -o "$SCRIPTPATH/test-generated/json"
echo "*** CLI TESTING rust"
cargo run -p type-sitter-cli "$SCRIPTPATH/../vendor/tree-sitter-rust/src/node-types.json" -o "$SCRIPTPATH/test-generated/rust"
echo "*** CLI PASSED"