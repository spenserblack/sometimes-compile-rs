#!/bin/sh

# Updates the example's modified time so that it gets recompiled, then compiles it.
rerun() {
	touch examples/example.rs && cargo run --example example
}

echo "Rerunning until compilation fails..."
while rerun; do
	:
done
