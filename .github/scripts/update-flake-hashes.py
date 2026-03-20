#!/usr/bin/env python3
"""Update flake.nix hashes for a new tmux-pomodoro release."""

import base64
import hashlib
import re
import sys
import urllib.request

TARGETS = {
    "x86_64-linux": "x86_64-unknown-linux-gnu",
    "aarch64-darwin": "aarch64-apple-darwin",
    "x86_64-darwin": "x86_64-apple-darwin",
}


def sri_hash(url: str) -> str:
    with urllib.request.urlopen(url) as resp:
        digest = hashlib.sha256(resp.read()).digest()
    return "sha256-" + base64.b64encode(digest).decode()


def main() -> None:
    tag = sys.argv[1]
    base = f"https://github.com/tmux-contrib/tmux-pomodoro/releases/download/{tag}"

    hashes = {
        nix_system: sri_hash(f"{base}/pomodoro-{rust_target}")
        for nix_system, rust_target in TARGETS.items()
    }

    with open("flake.nix") as f:
        content = f.read()

    for key, new_hash in hashes.items():
        content = re.sub(
            r'("' + re.escape(key) + r'"\s*=\s*")sha256-[^"]+',
            lambda m, h=new_hash: m.group(1) + h,
            content,
        )

    with open("flake.nix", "w") as f:
        f.write(content)

    for nix_system, h in hashes.items():
        print(f"  {nix_system}: {h}")


if __name__ == "__main__":
    main()
