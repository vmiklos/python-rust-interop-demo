#!/usr/bin/env bash

if ! cargo build; then
	echo "hint: https://doc.rust-lang.org/cargo/getting-started/installation.html"
	exit 1
fi

if ! ln -sf target/debug/libstring_sum.so string_sum.so; then
	echo "symlink failed"
	exit 1
fi

if ! python3 test.py; then
	echo "python failed"
	exit 1
fi

echo "If you saw '3' above, then all is OK."
