# Contributing

Thanks for helping build **Scoop** — an AI-powered visual workspace for Linux.

## Before you code

1. Read [docs/product/vision.md](./docs/product/vision.md) — especially the product principle
2. Skim [docs/product/mvp-scope.md](./docs/product/mvp-scope.md) so work stays in scope
3. Use the matching [feature spec](./docs/features/) for behavior and acceptance criteria
4. Follow [docs/engineering/coding-standards.md](./docs/engineering/coding-standards.md)

## Dev environment

See [docs/engineering/development-setup.md](./docs/engineering/development-setup.md).

## Pull requests

- Keep PRs focused on one feature or fix
- Note display server (Wayland/X11) for capture/hotkey changes
- Include a short test plan
- Update docs/ADRs when behavior or architecture changes

## Product principle check

> Can the user get from something visible on their screen to a useful result in seconds?

If your change adds steps, friction, or permanent screenshot retention by default — reconsider.
