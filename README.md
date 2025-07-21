# Notes
Interestingly, [Github's starter-workflows](https://github.com/actions/starter-workflows/tree/main), at least for these ones related to Rust, don't actually work on their own service.

## Changes
*Note: For this section, prefix each file with `.github/workflows/`.

- Actually installed the Rust toolchain in `rust.yml`, which for some reason was missing even though `rust-clippy.yml` had it.
- Changed `$cron-weekly` to `'0 14 * * 1'`, as `$cron-weekly` was not recognized.
