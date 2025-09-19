# nw

Nix wrapper, inspired from [nh](https://github.com/nix-community/nh)

## What?

- **Nice ui** thanks to [nix-output-monitor](https://code.maralorn.de/maralorn/nix-output-monitor) and [nvd](https://khumba.net/projects/nvd)
- **Enhance some of the `nix` commands**: because who wants to write `nixpkgs#` each time trying a package?
- **Opinionated API**: but when we think about it, aren't every API opinionated in some way?

## How?

### Nix

To use it with Nix, you just have to run one command

```bash
nix shell github:polnio/nw
```

Replace `shell` with `run` or `build` depending on your needs. And if you add it in the `environment.systemPackages` of your `configuration.nix`, you will get the man page and completions for bash, zsh and fish for free

### Cargo

Like every rust project should, you can build it with one `cargo` command

```bash
cargo build --release
```

Then add it in your `PATH` in any way you want.

## Why?

Because I find the `nix` command aweful. Because `nh`, despite being very good, doesn't fit my workflow. And, of course, because I could do it.

## Thanks a lot!

- Thanks to [nh](https://github.com/nix-community/nh) for the inspiration, and for some part of the code (especially for the [searching part](https://github.com/nix-community/nh/blob/df9fdd6ac3f8bf5ea6f241ec5bb44d1a36e70272/src/search.rs))
- Thanks to [nix-output-monitor](https://code.maralorn.de/maralorn/nix-output-monitor) and [nvd](https://khumba.net/projects/nvd) for the ui
- Thanks to [nix-index](https://github.com/nix-community/nix-index) for the `locate` feature
