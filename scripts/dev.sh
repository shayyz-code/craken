#!/bin/bash
set -e

echo "🚀 Craken Framework Automation Script"

case "$1" in
    "test")
        echo "🧪 Running tests..."
        cargo test --workspace
        ;;
    "check")
        echo "🔍 Checking compilation..."
        cargo check --workspace
        ;;
    "fmt")
        echo "🎨 Formatting code..."
        cargo fmt --all
        ;;
    "clippy")
        echo "📎 Running clippy..."
        cargo clippy --workspace -- -D warnings
        ;;
    "run-example")
        echo "🏃 Running example app..."
        cargo run --example craken-app -- serve
        ;;
    *)
        echo "Usage: $0 {test|check|fmt|clippy|run-example}"
        exit 1
        ;;
esac
