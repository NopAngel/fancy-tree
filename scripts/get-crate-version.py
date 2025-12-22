#!/usr/bin/env python3
"""
Prints the crate version to stdout without a trailing newline.
"""
import tomllib
from pathlib import Path

ROOT = Path(__file__).parent.parent
CARGO_TOML = ROOT / "Cargo.toml"

with open(CARGO_TOML, "rb") as fp:
    manifest = tomllib.load(fp)

version = manifest["package"]["version"]
print(version, end="")
