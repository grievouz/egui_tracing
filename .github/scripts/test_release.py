import contextlib
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import subprocess
import tarfile
import tempfile
import unittest
import urllib.error

spec = importlib.util.spec_from_file_location("release", Path(__file__).with_name("release.py"))
release = importlib.util.module_from_spec(spec)
spec.loader.exec_module(release)


class TagChecks(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.chdir = contextlib.chdir(self.root)
        self.chdir.__enter__()
        self.addCleanup(self.chdir.__exit__, None, None, None)
        self.git("init", "-q")
        self.git("config", "user.name", "Release fixture")
        self.git("config", "user.email", "fixture@example.invalid")
        self.git("config", "commit.gpgsign", "false")
        self.git("config", "tag.gpgsign", "false")
        (self.root / "egui-tracing").mkdir()
        self.manifest = self.root / "egui-tracing/Cargo.toml"
        self.manifest.write_text('[package]\nname = "egui_tracing"\nversion = "0.4.0"\n')
        self.git("add", ".")
        self.git("commit", "-qm", "fixture")
        self.sha = self.git("rev-parse", "HEAD")
        self.git("update-ref", "refs/remotes/origin/main", self.sha)
        self.git("tag", "egui_tracing-v0.4.0")
        self.env = {"GITHUB_REPOSITORY": release.REPOSITORY, "GITHUB_SHA": self.sha, "GITHUB_REF": "refs/tags/egui_tracing-v0.4.0"}

    def git(self, *args):
        return subprocess.check_output(["git", *args], stderr=subprocess.DEVNULL, text=True).strip()

    def check(self):
        return release.validate(self.root, self.env)

    def test_stable_lightweight_tag(self):
        self.assertEqual(self.check(), {"version": "0.4.0", "sha": self.sha, "tag": "egui_tracing-v0.4.0"})

    def test_unexpected_repository(self):
        self.env["GITHUB_REPOSITORY"] = "other/egui_tracing"
        with self.assertRaises(ValueError):
            self.check()

    def test_annotated_tag(self):
        self.git("tag", "-d", "egui_tracing-v0.4.0")
        self.git("tag", "-a", "egui_tracing-v0.4.0", "-m", "fixture")
        self.assertEqual(self.check()["sha"], self.sha)

    def test_malformed_or_prerelease_tag(self):
        for ref in ["refs/heads/main", "refs/tags/v0.4.0", "refs/tags/egui_tracing-v0.4.0-rc1", "refs/tags/egui_tracing-v00.4.0", "refs/tags/egui_tracing-v0.4.0\n"]:
            with self.subTest(ref=ref), self.assertRaises(ValueError):
                self.env["GITHUB_REF"] = ref
                self.check()

    def test_version_mismatch(self):
        self.manifest.write_text(self.manifest.read_text().replace("0.4.0", "0.3.0"))
        with self.assertRaises(ValueError):
            self.check()

    def test_package_name_mismatch(self):
        self.manifest.write_text(self.manifest.read_text().replace("egui_tracing", "other"))
        with self.assertRaises(ValueError):
            self.check()

    def new_commit(self):
        (self.root / "new").write_text("new")
        self.git("add", ".")
        self.git("commit", "-qm", "next fixture")
        return self.git("rev-parse", "HEAD")

    def test_checkout_mismatch(self):
        self.new_commit()
        with self.assertRaises(ValueError):
            self.check()

    def test_moved_tag(self):
        self.env["GITHUB_SHA"] = self.new_commit()
        with self.assertRaises(ValueError):
            self.check()

    def test_tag_not_on_main(self):
        self.env["GITHUB_SHA"] = self.new_commit()
        self.git("tag", "-f", "egui_tracing-v0.4.0")
        with self.assertRaises(subprocess.CalledProcessError):
            self.check()

    def test_old_ancestor_on_main_is_allowed(self):
        new = self.new_commit()
        self.git("update-ref", "refs/remotes/origin/main", new)
        self.git("checkout", "--detach", self.sha)
        self.assertEqual(self.check()["sha"], self.sha)

    def test_legacy_publisher_rejected(self):
        path = self.root / ".github/workflows/release-plz.yml"
        path.parent.mkdir(parents=True)
        path.write_text("legacy")
        with self.assertRaises(ValueError):
            self.check()


class RegistryChecks(unittest.TestCase):
    def response(self, checksum="abc", version="0.4.0", yanked=False):
        return io.BytesIO(json.dumps({"version": {"num": version, "checksum": checksum, "yanked": yanked}}).encode())

    def test_checksum_match(self):
        release.verify_publication("0.4.0", "abc", opener=lambda *a, **kw: self.response())

    def test_fail_closed_mismatch(self):
        for kwargs in [{"checksum": "other"}, {"version": "0.3.0"}, {"yanked": True}]:
            with self.subTest(kwargs=kwargs), self.assertRaises(ValueError):
                release.verify_publication("0.4.0", "abc", opener=lambda *a, **kw: self.response(**kwargs))

    def test_propagation_retry(self):
        calls = []
        def opener(*args, **kwargs):
            calls.append(1)
            if len(calls) == 1:
                raise urllib.error.HTTPError("url", 404, "pending", None, None)
            return self.response()
        release.verify_publication("0.4.0", "abc", attempts=2, pause=lambda _: None, opener=opener)
        self.assertEqual(len(calls), 2)

    def test_propagation_timeout(self):
        def missing(*args, **kwargs):
            raise urllib.error.HTTPError("url", 404, "pending", None, None)
        with self.assertRaises(ValueError):
            release.verify_publication("0.4.0", "abc", attempts=2, pause=lambda _: None, opener=missing)

    def test_other_registry_errors_abort(self):
        def denied(*args, **kwargs):
            raise urllib.error.HTTPError("url", 403, "denied", None, None)
        with self.assertRaises(urllib.error.HTTPError):
            release.verify_publication("0.4.0", "abc", opener=denied)

    def test_archive_source_and_checksum(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "egui_tracing-0.4.0.crate"
            def archive(**kwargs):
                info = {"git": {"sha1": "abc", **kwargs}, "path_in_vcs": "egui-tracing"}
                data = json.dumps(info).encode()
                with tarfile.open(path, "w:gz") as output:
                    member = tarfile.TarInfo("egui_tracing-0.4.0/.cargo_vcs_info.json")
                    member.size = len(data)
                    output.addfile(member, io.BytesIO(data))
            archive()
            self.assertEqual(release.archive_checksum(path, "abc"), hashlib.sha256(path.read_bytes()).hexdigest())
            with self.assertRaises(ValueError):
                release.archive_checksum(path, "wrong")
            archive(dirty=True)
            with self.assertRaises(ValueError):
                release.archive_checksum(path, "abc")


if __name__ == "__main__":
    unittest.main()
