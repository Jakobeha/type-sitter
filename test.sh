set -e
echo "*** TESTING yak-sitter"
cargo test -p yak-sitter
echo "*** TESTING type-sitter-gen"
cargo test -p type-sitter-gen
echo "*** TESTING type-sitter-lib"
cargo test -p type-sitter-lib
echo "*** TESTING type-sitter-lib with yak-sitter"
cargo test -p type-sitter-lib --features yak-sitter
echo "*** TESTING type-sitter-proc"
cargo test -p type-sitter-proc-tests
echo "*** TESTING type-sitter-cli"
cargo build -p type-sitter-cli && ./type-sitter-cli/test.sh
echo "*** PASSED"