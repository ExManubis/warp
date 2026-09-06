# Contributing

Thanks for helping improve PrompTTY.

## Development

```bash
./script/bootstrap   # platform-specific setup
./script/run         # build and run the GUI
./script/run-tui     # build and run the TUI
./script/presubmit   # fmt, clippy, and tests
```

See `AGENTS.md` for more workspace commands.

## Pull requests

- Open an issue first for anything larger than a small bug fix.
- Include a short description of what changed and how you tested it.
- Run `./script/presubmit` before you push.
- Manual testing with `./script/run` is expected for user-visible changes.

## Security

Do not file public issues for vulnerabilities. See [SECURITY.md](SECURITY.md).
