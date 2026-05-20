# Installation

Currently the preferred way of installing floo is through [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) by running

```sh
cargo install floo
```

Afterwards, add the following to your shell's rc file (.bashrc/.zshrc):

```sh
eval "$(floo-bin init)"
```

And restart your terminal.

*Note: We are planning to release to other package managers in the future once floo becomes more stable.*

## Building from source

To build from source, the same workflow applies via a local cargo installation flow:

```sh
git clone https://github.com/keyvizsla/floo.git
cargo install --path floo
echo 'eval "$(floo-bin init)"' >> ~/.bashrc
```

Make sure to also restart your terminal for the local installation to take effect.
