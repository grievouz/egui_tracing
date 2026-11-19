# Development and releases

Development targets `main`. During the branch transition, CI also accepts `dev`.
Published versions are snapshots identified by `egui_tracing-vX.Y.Z` tags; there
is no release-only branch and no publisher triggered by branch pushes.

## One-time migration

Perform the following in order; do not advance the old `main` first.

1. Disable the legacy **Release Plz** workflow and verify its disabled state.
   It publishes on pushes to the old `main`. Keep it disabled throughout the
   cutover, including any rollback.
2. Review and merge the workflow migration into `dev`. Verify native and wasm
   CI on the exact resulting commit. The migration removes the old publisher,
   adds tag validation, and keeps CI on both branch names temporarily.
3. Re-fetch both remote heads. Require `main` to be an ancestor of the reviewed,
   tested `dev` commit. If either head changed, re-evaluate before proceeding.
4. Fast-forward `main` to that exact commit with an ordinary non-force push.
   Verify the remote head and native, wasm, and Pages workflows. A branch push
   must not start the new tag-only release workflow.
5. Change the default branch to `main`. Check open PR bases, branch rules,
   Pages environment policy, and external links. Keep `dev` until its removal
   is explicitly requested; do not delete historical release tags.

If the ancestry check fails, preserve intervening history and prepare a reviewed
integration commit instead of resetting either branch. If a build fails after
cutover, fix forward; do not restore the old publisher or rewind `main`.

## Publishing setup (separate from the branch migration)

Publishing is **off by default**. Tags can run all validation while the
repository variable `RELEASE_PUBLISH_ENABLED` is absent or not `true`.
Before setting it to `true`:

- Configure the `crates-io` environment with required human review, no
  self-approval if supported, and deployment access restricted to release tags.
  Verify those protections exist; merely naming an environment in YAML does
  not create an approval gate. If the account cannot enforce that gate, leave
  publishing disabled until an alternative approval mechanism is reviewed.
- Add an environment-scoped `CARGO_REGISTRY_TOKEN` restricted to publishing
  `egui_tracing`. Do not reuse a broad account token. The workflow exposes it
  only to the actual `cargo publish` step. Trusted publishing can replace this
  token in a separately reviewed change once configured on crates.io.
- Protect release tags against updates/deletion and restrict who can create
  them. Configure appropriate `main` review/check requirements separately.
- Validate the publishing account and crate ownership without logging secrets.
  Remove obsolete release-plz credentials only after checking for other uses.

## Releasing

1. Prepare a version-bump PR and concise technical release notes. Choose the
   version based on the complete change set: the pending egui 0.34 to 0.36
   upgrade warrants 0.4.0 rather than a 0.3.x patch. Do not bump or publish a
   version solely to test this migration.
2. Merge the reviewed PR, then verify CI on the exact `main` commit. Re-fetch
   it before creating a tag. Confirm the package manifest version and that the
   version is not already published. Only release one version at a time.
3. Following release approval, create and push an annotated tag of that exact
   SHA named `egui_tracing-vX.Y.Z`. Never move/reuse a published tag.
4. The tag workflow rejects malformed/prerelease tags, version mismatches,
   commits not on `main`, and mismatched tag/event/checkout commits. It runs
   native, audit, wasm, and release-guard tests and a credential-free package
   dry run. The package job explicitly generates a workspace lockfile before
   running the locked dry run and packaging; library packaging alone does not
   create a root lockfile. It records the clean source SHA and archive checksum.
5. After environment approval, the publishing job revalidates the tag and
   ancestry, uses the validation job's lockfile, rebuilds the package, and
   requires an identical checksum before exposing credentials. It publishes
   only `egui_tracing`, verifies the crates.io checksum, then creates a GitHub
   release with a package link. Review/edit richer technical release notes
   separately if desired.
6. Verify crates.io, docs.rs, the GitHub tag/release, and downstream compatibility.

The lockfile is shared between package validation and publication within a run;
this does not claim the whole project's dependency resolution is permanently
locked. Native and wasm CI currently resolve dependencies independently.

## Failure and recovery

No step treats an already-published version as automatically safe. Registry
publication is immutable; never overwrite a version or blindly skip a failed
publish. If publication succeeds but verification or GitHub release creation
fails, compare the registry checksum with the recorded package checksum and
source SHA before separately completing release metadata. Rerunning publication
for an existing version will fail rather than silently publish different code.
If code must change, prepare a new version and a new reviewed tag.

For migration validation, use local fixture repositories and dry runs. Do not
push a disposable release tag: it is a publishing trigger once enabled.
