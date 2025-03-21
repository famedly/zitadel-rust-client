# Zitadel Rust Client

[![rust workflow status][badge-rust-workflow-img]][badge-rust-workflow-url]
[![docker workflow status][badge-docker-workflow-img]][badge-docker-workflow-url]
[![docs main][badge-docs-main-img]][badge-docs-main-url]

[badge-rust-workflow-img]: https://github.com/famedly/rust-library-template/actions/workflows/rust.yml/badge.svg
[badge-rust-workflow-url]: https://github.com/famedly/rust-library-template/commits/main
[badge-docker-workflow-img]: https://github.com/famedly/rust-library-template/actions/workflows/docker.yml/badge.svg
[badge-docker-workflow-url]: https://github.com/famedly/rust-library-template/commits/main
[badge-docs-main-img]: https://img.shields.io/badge/docs-main-blue
[badge-docs-main-url]: https://famedly.github.io/rust-library-template/project_name/index.html

A zitadel client written in rust

## Zitadel Version

The required and tested version of Zitadel is **v2.61.2**

## Adding new endpoints

The models behind each method are generated using
[swagger-codegen](https://github.com/swagger-api/swagger-codegen),
then the top-level `Zitadel` methods are all hand-written on top of
this to tape over the ergonomics issues codegen causes (e.g. by
providing `Stream`s instead of manual pagination).

To add a new model:

1. Download the correct tag of
[Zitadel](https://github.com/zitadel/zitadel/)
2. Install build dependencies
  - Primarily `go`, `buf` and `make`
  - `nix develop github:NixOS/nixpkgs-unstable#zitadel` gives good results
3. Ensure `PATH=$PATH:$HOME/go/bin`, or some alternate
   `$GOPATH`/`$GOBIN` is set
3. Run `make core_api` in the Zitadel repository
4. Create a new subdirectory for the new endpoints
5. Run `swagger-codegen generate -i <path-to-repo>/openapi/<path-to-API>.json -l rust`
  - Individual `.json` files *can* be used
6. Take the `models` directory out of the resulting Rust repo, and
   discard any other generated code.
7. Use `sed` and similar to clean up any fallout; adding `#![allow]s`
   to the generated `mod.rs` is also helpful.

## Lints

```sh
cargo clippy --workspace --all-targets
```

and this in your IDE:
```sh
cargo clippy --workspace --all-targets --message-format=json
```

## Pre-commit usage

1. If not installed, install with your package manager, or `pip install --user pre-commit`
2. Run `pre-commit autoupdate` to update the pre-commit config to use the newest template
3. Run `pre-commit install` to install the pre-commit hooks to your local environment

---

# Famedly

**This project is part of the source code of Famedly.**

We think that software for healthcare should be open source, so we publish most
parts of our source code at [github.com/famedly](https://github.com/famedly).

Please read [CONTRIBUTING.md](CONTRIBUTING.md) for details on our code of
conduct, and the process for submitting pull requests to us.

For licensing information of this project, have a look at the [LICENSE](LICENSE.md)
file within the repository.

If you compile the open source software that we make available to develop your
own mobile, desktop or embeddable application, and cause that application to
connect to our servers for any purposes, you have to agree to our Terms of
Service. In short, if you choose to connect to our servers, certain restrictions
apply as follows:

- You agree not to change the way the open source software connects and
  interacts with our servers
- You agree not to weaken any of the security features of the open source software
- You agree not to use the open source software to gather data
- You agree not to use our servers to store data for purposes other than
  the intended and original functionality of the Software
- You acknowledge that you are solely responsible for any and all updates to
  your software

No license is granted to the Famedly trademark and its associated logos, all of
which will continue to be owned exclusively by Famedly GmbH. Any use of the
Famedly trademark and/or its associated logos is expressly prohibited without
the express prior written consent of Famedly GmbH.

For more
information take a look at [Famedly.com](https://famedly.com) or contact
us by [info@famedly.com](mailto:info@famedly.com?subject=[GitLab]%20More%20Information%20)
