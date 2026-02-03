#!/usr/bin/env python3
import hashlib
import json
import shutil
import subprocess
import sys
from pathlib import Path

EXPECTED_SHA256 = "5e735f6e44f56e9eee91e5626252663afcc5263287d1c5980367b3f9f930a0e8"


def sha256(path: Path) -> str:
    hasher = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(1024 * 1024), b""):
            hasher.update(chunk)
    return hasher.hexdigest()


def main() -> int:
    repo_root = Path(__file__).resolve().parents[1]
    source_path = repo_root / "resources" / "sp1" / "vk_map.bin"

    if not source_path.exists():
        print(f"Missing {source_path}.", file=sys.stderr)
        return 1

    source_hash = sha256(source_path)
    if source_hash != EXPECTED_SHA256:
        print(
            f"vk_map.bin hash mismatch: expected {EXPECTED_SHA256}, got {source_hash}.",
            file=sys.stderr,
        )
        return 1

    metadata_raw = subprocess.check_output(
        ["cargo", "metadata", "--format-version", "1"],
        cwd=repo_root,
    )
    metadata = json.loads(metadata_raw)

    sp1_packages = [pkg for pkg in metadata.get("packages", []) if pkg.get("name") == "sp1-prover"]
    if not sp1_packages:
        print("sp1-prover package not found in cargo metadata.", file=sys.stderr)
        return 1

    dest_paths = {
        (Path(pkg["manifest_path"]).resolve().parent / "src" / "vk_map.bin")
        for pkg in sp1_packages
    }

    for dest_path in sorted(dest_paths):
        if dest_path.exists() and sha256(dest_path) == EXPECTED_SHA256:
            print(f"vk_map.bin already installed at {dest_path}")
            continue

        dest_path.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(source_path, dest_path)

        dest_hash = sha256(dest_path)
        if dest_hash != EXPECTED_SHA256:
            print(
                f"Failed to install vk_map.bin at {dest_path}: hash mismatch ({dest_hash}).",
                file=sys.stderr,
            )
            return 1

        print(f"Installed vk_map.bin at {dest_path}")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
