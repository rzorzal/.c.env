#!/bin/bash
# Test script for --dry flag functionality

echo "======================================"
echo "C.env Compiler - --dry Flag Tests"
echo "======================================"
echo ""

# Cleanup
rm -f .env .env.production .env.staging .env.test

echo "Test 1: Default mode with --module=production"
echo "----------------------------------------------"
echo "Command: cenv examples/config.cenv --module=production"
echo ""
./target/release/cenv examples/config.cenv --module=production 2>&1
echo ""
echo "File created: .env.production"
ls -lh .env.production
echo ""
echo "First 5 lines:"
head -5 .env.production
echo ""
echo ""

echo "Test 2: Dry run mode with --module=staging"
echo "-------------------------------------------"
echo "Command: cenv examples/config.cenv --module=staging --dry"
echo ""
./target/release/cenv examples/config.cenv --module=staging --dry 2>&1 | head -10
echo ""
if [ -f .env.staging ]; then
    echo "❌ FAILED: .env.staging file was created (should not exist in dry run)"
else
    echo "✓ PASSED: No .env.staging file created"
fi
echo ""
echo ""

echo "Test 3: Default mode without --module"
echo "--------------------------------------"
echo "Command: cenv examples/test_public_vars.cenv"
echo ""
./target/release/cenv examples/test_public_vars.cenv 2>&1
echo ""
echo "File created: .env"
ls -lh .env
echo ""
echo "Content:"
cat .env
echo ""
echo ""

echo "Test 4: Dry run without --module"
echo "---------------------------------"
echo "Command: cenv examples/test_public_vars.cenv --dry"
echo ""
rm -f .env
./target/release/cenv examples/test_public_vars.cenv --dry 2>&1 | head -10
echo ""
if [ -f .env ]; then
    echo "❌ FAILED: .env file was created (should not exist in dry run)"
    rm -f .env
else
    echo "✓ PASSED: No .env file created"
fi
echo ""
echo ""

echo "Test 5: Help text shows --dry flag"
echo "-----------------------------------"
./target/release/cenv --help | grep -A2 "dry"
echo ""
echo ""

echo "======================================"
echo "All tests completed!"
echo "======================================"
echo ""
echo "Generated files:"
ls -lh .env* 2>/dev/null || echo "No .env files in current directory"
