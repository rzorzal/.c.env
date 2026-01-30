#!/bin/bash
# Test script for --output flag functionality

echo "======================================"
echo "C.env Compiler - --output Flag Tests"
echo "======================================"
echo ""

# Cleanup
rm -f .env .env.production .env.staging .env.custom .env.prod .env.test

echo "Test 1: Default mode (no flags)"
echo "--------------------------------"
echo "Command: cenv examples/test_public_vars.cenv"
echo "Expected: Creates .env"
echo ""
./target/release/cenv examples/test_public_vars.cenv 2>&1 | head -3
ls -lh .env 2>/dev/null && echo "✓ PASSED: .env created" || echo "❌ FAILED"
echo ""
echo ""

echo "Test 2: --module=production"
echo "----------------------------"
echo "Command: cenv examples/config.cenv --module=production"
echo "Expected: Creates .env.production"
echo ""
./target/release/cenv examples/config.cenv --module=production 2>&1 | head -3
ls -lh .env.production 2>/dev/null && echo "✓ PASSED: .env.production created" || echo "❌ FAILED"
echo ""
echo ""

echo "Test 3: --output=.env.custom"
echo "-----------------------------"
echo "Command: cenv examples/test_public_vars.cenv --output=.env.custom"
echo "Expected: Creates .env.custom"
echo ""
./target/release/cenv examples/test_public_vars.cenv --output=.env.custom 2>&1 | head -3
ls -lh .env.custom 2>/dev/null && echo "✓ PASSED: .env.custom created" || echo "❌ FAILED"
echo ""
echo ""

echo "Test 4: --module=staging --output=.env.prod (output overrides module)"
echo "-----------------------------------------------------------------------"
echo "Command: cenv examples/config.cenv --module=staging --output=.env.prod"
echo "Expected: Creates .env.prod with staging content, NOT .env.staging"
echo ""
rm -f .env.staging .env.prod
./target/release/cenv examples/config.cenv --module=staging --output=.env.prod 2>&1 | head -3
echo ""
if [ -f .env.prod ] && [ ! -f .env.staging ]; then
    echo "✓ PASSED: .env.prod created, .env.staging not created"
    echo "Content verification (should have staging values):"
    grep "staging" .env.prod && echo "✓ Contains staging URL" || echo "❌ Missing staging URL"
else
    echo "❌ FAILED"
    ls -la .env.prod .env.staging 2>&1
fi
echo ""
echo ""

echo "Test 5: Priority test - output > module > default"
echo "--------------------------------------------------"
echo "Test 5a: Only --output"
rm -f .env.test
./target/release/cenv examples/test_public_vars.cenv --output=.env.test 2>&1 | head -1
[ -f .env.test ] && echo "✓ .env.test created" || echo "❌ Failed"
echo ""

echo "Test 5b: --module without --output"
rm -f .env.staging
./target/release/cenv examples/config.cenv --module=staging 2>&1 | head -1
[ -f .env.staging ] && echo "✓ .env.staging created" || echo "❌ Failed"
echo ""

echo "Test 5c: Neither --output nor --module"
rm -f .env
./target/release/cenv examples/test_public_vars.cenv 2>&1 | head -1
[ -f .env ] && echo "✓ .env created" || echo "❌ Failed"
echo ""
echo ""

echo "Test 6: Help text shows --output flag"
echo "--------------------------------------"
./target/release/cenv --help | grep -A1 "output="
echo ""
echo ""

echo "======================================"
echo "All tests completed!"
echo "======================================"
echo ""
echo "Generated files:"
ls -lh .env* 2>/dev/null | awk '{print $9, "-", $5}' || echo "No .env files in current directory"
echo ""
echo "Cleanup:"
rm -f .env .env.production .env.staging .env.custom .env.prod .env.test
echo "✓ Test files removed"
