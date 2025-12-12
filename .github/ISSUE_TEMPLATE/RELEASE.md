---
name: New Release
about: Propose a new release
title: Release v0.x.0
labels: ''
assignees: ''

---

## Release Checklist
<!--
Please do not remove items from the checklist
-->
- [ ] Verify that the changelog in this issue is up-to-date
- [ ] Bump the version number in `pyproject.toml` to the new version (e.g., v0.x.0)
- [ ] Merge the PR that bumps the version number
- [ ] For major or minor releases (v$MAJ.$MIN.0), create a new release branch.
  - [ ] an OWNER creates a vanilla release branch with
        `git branch release-$MAJ.$MIN.0 main`
  - [ ] An OWNER pushes the new release branch with
        `git push --set-upstream upstream release-$MAJ.$MIN.0`
- [ ] An OWNER [prepares a draft release](https://github.com/inftyai/amrs/releases)
  - [ ] Write the change log into the draft release.
  - [ ] Don't release the draft yet.
- [ ] Publish the release to PyPI
    - [ ] run `make build` to build the package
    - [ ] run `make publish` to publish the package to PyPI
- [ ] Publish the draft release prepared at the [Github releases page](https://github.com/inftyai/amrs/releases).
- [ ] Close this issue


## Changelog
<!--
Describe changes since the last release here.
-->
