# Contributing to setupercent

Thanks for taking the time to make setupercent better. Whether it's a bug report, a suggestion, or a pull request — every contribution counts.

## Reporting bugs

Open a new issue using the [bug report template](.github/ISSUE_TEMPLATE/bug_report.md). The more details you provide, the faster it can be fixed. Please include:

- Windows version
- setupercent version (shown in the title bar)
- The device model that is affected

## Suggesting features

Use the [feature request template](.github/ISSUE_TEMPLATE/feature_request.md). Describe the problem you want solved, not just the solution — it leaves room for better ideas.

## Pull requests

1. Fork the repository and create a branch from `main`.
2. Keep the change focused. One logical change per pull request.
3. Match the existing code style. The project uses standard Rust formatting:
   ```bash
   cargo fmt
   cargo clippy -- -D warnings
   ```
4. Make sure everything builds:
   ```bash
   cargo build --release
   ```
5. Write a clear pull request title and describe what was changed and why.

## Development setup

```bash
git clone https://github.com/jocray/setupercent
cd setupercent
cargo build --release
```

The resulting binary is at `target/release/setupercent.exe`.

## Commit messages

Short, imperative, present tense — `fix tray icon flicker`, `add Russian translation`, `refactor battery polling`.

## Code of conduct

Be respectful. Assume good faith. Keep discussions focused on the project.

## License

By contributing, you agree that your contributions are released under the same [MIT License](LICENSE) that covers the project.