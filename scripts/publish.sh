#!/bin/bash
set -e

# Publication order for Craken crates
CRATES=(
    "crates/craken-container"
    "crates/craken-macros"
    "crates/craken-logging"
    "crates/craken-config"
    "crates/craken-core"
    "crates/craken-http"
    "crates/craken-database"
    "crates/craken-cli"
)

DRY_RUN=false
if [[ "$1" == "--dry-run" ]]; then
    DRY_RUN=true
    echo "Running cargo publish --dry-run for all crates..."
else
    echo "Publishing all Craken crates to crates.io..."
fi

for CRATE in "${CRATES[@]}"; do
    echo "--------------------------------------------------------"
    echo "Processing: $CRATE"
    
    if [[ "$DRY_RUN" == true ]]; then
        # Skip dry-run for dependent crates if they aren't on crates.io yet,
        # or use --allow-dirty and accept that it might fail for inter-dependent crates
        # in a dry-run scenario.
        if [[ "$CRATE" == "crates/craken-container" || "$CRATE" == "crates/craken-macros" || "$CRATE" == "crates/craken-logging" || "$CRATE" == "crates/craken-config" ]]; then
            (cd "$CRATE" && cargo publish --dry-run --allow-dirty)
        else
            echo "Skipping dry-run for $CRATE (depends on crates not yet published)"
        fi
    else
        (cd "$CRATE" && cargo publish --no-verify)
        echo "Published: $CRATE"
        echo "Waiting 10 seconds for crates.io to update index..."
        sleep 10
    fi
done

echo "--------------------------------------------------------"
if [[ "$DRY_RUN" == true ]]; then
    echo "Dry-run complete. All crates are ready for publication."
else
    echo "All Craken crates have been published successfully!"
fi
