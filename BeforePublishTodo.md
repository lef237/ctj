Before cargo publishing, check and perform the following:

Required check items

1. Check the Cargo.toml settings

- version, name, description
- license, repository, homepage
- authors or license-file

2. Test execution

- Pass all tests with `cargo test`
- Build Normal with `cargo build --release`

3. Check the document

- No errors in document generation with `cargo doc`
- README.md is up to date

4. Lint Format

- Code formatting with `cargo fmt`
- Check warnings with `cargo clippy`

5. Check the package contents

- Create and confirm distribution packages with `cargo package`
- Does not contain unnecessary files?

6. Version control

- Appropriate version number (semantic versioning)
  - Rerun `cargo build --release`
- CHANGELOG.md Update (recommended)

7. Commit and push

- Commit the changes with a meaningful message
- Push to the remote repository

8. Publish

- Publish the package with `cargo publish`
