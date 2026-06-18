#!/usr/bin/env bash
set -e

echo "Azure Policy Drift Detector Setup"
echo "====================================="
echo ""
echo "Prerequisites:"
echo "  - Rust 1.78+ (https://rustup.rs)"
echo "  - An Azure app registration with Reader and Policy Insights Data Reader roles"
echo "    See docs/azure_setup.md for step-by-step instructions."
echo ""

if [ ! -f .env ]; then
    cp .env.example .env
    echo "Created .env from .env.example. Fill in your credentials."
else
    echo ".env already exists."
fi

echo ""
echo "Build:"
echo "  cargo build --release"
echo ""
echo "Run:"
echo "  ./target/release/apdd scan"
echo "  ./target/release/apdd scan --min-severity high"
echo "  ./target/release/apdd export --format md --output report.md"
