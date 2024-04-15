#!/bin/bash
which cargo && cargo install --path ./ || echo "Cargo(Rust package manager) is not installed"
