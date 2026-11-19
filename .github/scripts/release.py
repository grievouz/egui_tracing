"""Fail-closed checks for stable egui_tracing tag releases (stdlib only)."""

import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tarfile
import time
import tomllib
import urllib.error
import urllib.request

PACKAGE = "egui_tracing"
REPOSITORY = "grievouz/egui_tracing"
VERSION = r"(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)"


def git(*args):
    return subprocess.check_output(["git", *args], text=True).strip()


def validate(root, env):
    if env.get("GITHUB_REPOSITORY") != REPOSITORY:
        raise ValueError("unexpected repository")
    match = re.fullmatch(r"refs/tags/egui_tracing-v(" + VERSION + r")", env.get("GITHUB_REF", ""))
    if not match:
        raise ValueError("expected a stable egui_tracing-vX.Y.Z tag")
    version = match.group(1)
    manifest = tomllib.loads((root / "egui-tracing/Cargo.toml").read_text())["package"]
    if manifest["name"] != PACKAGE or manifest["version"] != version:
        raise ValueError("tag and package name/version do not match")
    sha = git("rev-parse", env["GITHUB_SHA"] + "^{commit}")
    if git("rev-parse", "HEAD") != sha:
        raise ValueError("checkout does not match the event commit")
    if git("rev-parse", env["GITHUB_REF"] + "^{commit}") != sha:
        raise ValueError("tag does not match the event commit")
    subprocess.run(["git", "merge-base", "--is-ancestor", sha, "refs/remotes/origin/main"], check=True)
    if (root / ".github/workflows/release-plz.yml").exists():
        raise ValueError("legacy publisher is still present")
    return {"version": version, "sha": sha, "tag": "egui_tracing-v" + version}


def archive_checksum(path, sha):
    with tarfile.open(path, "r:gz") as archive:
        prefix = path.name.removesuffix(".crate")
        info = json.load(archive.extractfile(prefix + "/.cargo_vcs_info.json"))
        if info["git"]["sha1"] != sha or info["git"].get("dirty", False):
            raise ValueError("package source is not the clean tagged commit")
        if info["path_in_vcs"] != "egui-tracing":
            raise ValueError("unexpected package source directory")
    return hashlib.sha256(path.read_bytes()).hexdigest()


def verify_publication(version, expected_checksum, attempts=30, pause=time.sleep, opener=urllib.request.urlopen):
    url = f"https://crates.io/api/v1/crates/{PACKAGE}/{version}"
    for attempt in range(attempts):
        try:
            request = urllib.request.Request(url, headers={"User-Agent": "egui_tracing-release-verification"})
            with opener(request, timeout=30) as response:
                published = json.load(response)["version"]
            if published["num"] != version or published["checksum"] != expected_checksum or published["yanked"]:
                raise ValueError("registry version/checksum/state does not match the tested package")
            return
        except urllib.error.HTTPError as error:
            if error.code != 404:
                raise
        if attempt + 1 < attempts:
            pause(10)
    raise ValueError("published version did not become visible on crates.io")


def main():
    command = sys.argv[1]
    if command == "validate":
        result = validate(Path.cwd(), os.environ)
        with open(os.environ["GITHUB_OUTPUT"], "a") as output:
            for key, value in result.items():
                output.write(f"{key}={value}\n")
    elif command == "checksum":
        print(archive_checksum(Path(sys.argv[2]), sys.argv[3]))
    elif command == "verify-publication":
        verify_publication(sys.argv[2], sys.argv[3])
    else:
        raise ValueError("unknown release command")


if __name__ == "__main__":
    main()
