#!/bin/bash
# Test script to verify all examples run correctly

echo "╔════════════════════════════════════════════════════════════╗"
echo "║           Testing All ESA Examples                        ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo ""

# Array of all examples
examples=(
    "cap_theorem"
    "consistent_hashing"
    "crdt_counter"
    "partitioning_strategies"
    "raft_election"
    "raft_log_replication"
    "rebalancing_demo"
    "storage_basics"
    # "rocksdb_poc"  # Skipped: RocksDB compilation is slow
)

# Track results
passed=0
failed=0

# Test each example
for example in "${examples[@]}"; do
    echo "━━━ Testing: $example ━━━"
    
    if cargo run --example "$example" > /dev/null 2>&1; then
        echo "✅ PASS: $example"
        ((passed++))
    else
        echo "❌ FAIL: $example"
        ((failed++))
        # Show error details
        echo "Error details:"
        cargo run --example "$example" 2>&1 | tail -10
    fi
    echo ""
done

# Summary
echo "╔════════════════════════════════════════════════════════════╗"
echo "║                      Summary                              ║"
echo "╚════════════════════════════════════════════════════════════╝"
echo "Total examples: ${#examples[@]}"
echo "Passed: $passed"
echo "Failed: $failed"
echo ""

if [ $failed -eq 0 ]; then
    echo "🎉 All examples are working correctly!"
    exit 0
else
    echo "⚠️  Some examples failed. Please check the errors above."
    exit 1
fi
