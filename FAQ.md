# FAQ

## How do I build PrompTTY?

Host builds are macOS and Linux only. SSH into a Windows machine still works.

```bash
./script/bootstrap
./script/run
```

See [CONTRIBUTING.md](CONTRIBUTING.md) and `AGENTS.md`.

## Does PrompTTY talk to Warp's servers?

Not intentionally, once the local-only cleanup is finished. The OSS binary still has leftover Warp cloud wiring (login, Drive, Oz). That is being stripped. User-owned network — API keys, MCP servers, SSH — stays.

## How do I report a security issue?

See [SECURITY.md](SECURITY.md). Do not open a public issue.
