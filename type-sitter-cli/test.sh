set -e
SCRIPTPATH="$( cd -- "$(dirname "$0")" >/dev/null 2>&1 ; pwd -P )"
echo "*** CLI TESTING json"
cargo run -p type-sitter-cli "$SCRIPTPATH/../vendor/tree-sitter-json" -o "$SCRIPTPATH/test-generated"
echo "*** CLI TESTING rust"
cargo run -p type-sitter-cli "$SCRIPTPATH/../vendor/tree-sitter-rust" -o "$SCRIPTPATH/test-generated" --use-yak-sitter --wrapper-namespace "yak_sitter"
echo "*** CLI PASSED"