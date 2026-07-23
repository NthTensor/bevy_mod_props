# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 3.0.0 - 2026-07-23

- Updated to bevy `v0.19`.
- `Props` is now a component and can be attached to entities directly.
  The resource implementation has been moved to `GlobalProps`, which wraps and
  dereferences to `Props`, as required by bevy `v0.19` (`Resource` now
  implies `Component`). The extension traits work unchanged.

## 2.0.0 - 2026-01-28

- Moved from `ustr` to `estr` (a fork of `ustr`).
- Updated to bevy `v0.18`.
- Improved link following.

## 1.0.0 - 2025-11-23

- Factored out of another project and open-sourced.
