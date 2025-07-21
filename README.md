# Notes
Interestingly, [Github's starter-workflows](https://github.com/actions/starter-workflows/tree/main), at least for these ones related to Rust, don't actually work on their own service.

This is in addition to [this blog post that the included `$default-branch` does not actually connect to Github](https://github.blog/changelog/2020-07-22-github-actions-better-support-for-alternative-default-branch-names/), which is unfortunate because [Github's "Getting Started with Github Actions"](https://docs.github.com/en/actions/get-started/quickstart) does not mention that directly but instead requires you to dig through a link which I missed my first time around.

## Changes
*Note: For this section, prefix each file with `.github/workflows/`.

- Actually installed the Rust toolchain in `rust.yml`, which for some reason was missing even though `rust-clippy.yml` had it.
- Changed `$cron-weekly` to `'0 14 * * 1'`, as `$cron-weekly` was not recognized.
