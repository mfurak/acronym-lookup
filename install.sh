#!/bin/bash
which cargo2 && cargo install --path ./ || echo "Cargo(Rust package manager) is not installed"
