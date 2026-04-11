# Changelog

## [0.6.1](https://github.com/tmux-contrib/tmux-pomodoro/compare/v0.6.0...v0.6.1) (2026-04-11)


### Bug Fixes

* add postCreateCommand to restore nix volume permissions ([877360c](https://github.com/tmux-contrib/tmux-pomodoro/commit/877360c20b03c8a0993c415360f7bc6401b2cb6d))
* resolve rebase conflict in flake.nix ([c546201](https://github.com/tmux-contrib/tmux-pomodoro/commit/c546201aff6f8ae2eebe20f820df2b246f3f97d2))

## [0.6.0](https://github.com/tmux-contrib/tmux-pomodoro/compare/v0.5.1...v0.6.0) (2026-03-24)


### Features

* install pre-built binaries from GitHub releases in flake.nix ([dd08516](https://github.com/tmux-contrib/tmux-pomodoro/commit/dd085168e0517dadf1169814579e2c876dcef7cc))

## [0.5.1](https://github.com/tmux-contrib/tmux-pomodoro/compare/v0.5.0...v0.5.1) (2026-03-11)


### Bug Fixes

* retry hook spawn on ETXTBSY to handle deferred Linux fput ([8a78c29](https://github.com/tmux-contrib/tmux-pomodoro/commit/8a78c29b53a97ea8d11231db6d6613ca0afc0cf2))

## [0.5.0](https://github.com/tmux-contrib/tmux-pomodoro/compare/v0.4.0...v0.5.0) (2026-03-07)


### Features

* **nix:** add packages.default and Rust toolchain to root flake ([db1a4de](https://github.com/tmux-contrib/tmux-pomodoro/commit/db1a4de4b2870fef15b7442fa0b8de7a1f7ffbfb))

## [0.4.0](https://github.com/tmux-contrib/tmux-pomodoro/compare/v0.3.0...v0.4.0) (2026-03-06)


### Features

* add native pomodoro CLI and migrate tmux plugin ([05a01ca](https://github.com/tmux-contrib/tmux-pomodoro/commit/05a01ca72f88c34820835af87039487060e5c50f))
* **build:** add Nix flake configuration ([4f2bf48](https://github.com/tmux-contrib/tmux-pomodoro/commit/4f2bf482eb8b9cfb162e81fe7daa98c0acf296e5))
* **keybindings:** add tmux keybindings for pomodoro control ([846f6c9](https://github.com/tmux-contrib/tmux-pomodoro/commit/846f6c98eb16a9e8e258cda33ba2f9966751a1cb))
* **notifications:** add configuration option to disable notifications ([70cf6ef](https://github.com/tmux-contrib/tmux-pomodoro/commit/70cf6efca49e6c17b90ca08d3947e6bac413eb6d))
* **pomodoro:** add configurable sub-keys for focus, break, and stop ([6e575d4](https://github.com/tmux-contrib/tmux-pomodoro/commit/6e575d4472eadc93285c10ff8706c02b0a64bfda))
* **pomodoro:** add interactive duration selection menus ([874d040](https://github.com/tmux-contrib/tmux-pomodoro/commit/874d04092628d0e94276f16d3e86b6c3cc760e9d))


### Bug Fixes

* **ci:** add id-token: write permission for release-please ([2934cd3](https://github.com/tmux-contrib/tmux-pomodoro/commit/2934cd364df84b79ad1570ac85ff9f6bb7714915))
* **ci:** add issues write permission for release-please ([116fb56](https://github.com/tmux-contrib/tmux-pomodoro/commit/116fb560e9caa99395e026366982374dc2eb2d91))
* **ci:** bump crates/pomodoro/Cargo.toml version via extra-files ([d9dcd55](https://github.com/tmux-contrib/tmux-pomodoro/commit/d9dcd5567d3d684466d41821432b59aef962f11f))
* **ci:** release entire repo at root instead of crate path ([d4303bc](https://github.com/tmux-contrib/tmux-pomodoro/commit/d4303bc974afd8ee3539964e22792a4dc964655e))
* **ci:** remove issues: write, keep id-token: write for release-please ([d42fee3](https://github.com/tmux-contrib/tmux-pomodoro/commit/d42fee3e43708e5bea4b3ab02eb22ef29098ac60))
* **ci:** restore issues: write permission for release-please ([d204f21](https://github.com/tmux-contrib/tmux-pomodoro/commit/d204f2198db39a1b89725733f6516ddcfb081107))
* **pomodoro:** position menus at right edge and status line ([3760942](https://github.com/tmux-contrib/tmux-pomodoro/commit/3760942465eff5a23ac58b25ba4a0721f44ce8b9))
* **tmux_core:** suppress command output in key bindings ([d9fcd38](https://github.com/tmux-contrib/tmux-pomodoro/commit/d9fcd38014881561a42303de6580433b7aecacc5))
* **tmux-cmd:** adjust break menu minimum duration to 5 minutes ([54db4ee](https://github.com/tmux-contrib/tmux-pomodoro/commit/54db4ee85859c7899607db5e41dae8fa12749617))
* **tmux:** correct script path and break menu shortcuts ([8238d0c](https://github.com/tmux-contrib/tmux-pomodoro/commit/8238d0c12ada9a414b46b373c0ae1737aa3b01c1))

## Changelog
