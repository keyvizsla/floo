# Installation

## Prerequisites

Currently the preferred way of installing *floo* is through [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) by running

```sh
cargo install floo
```
*Note: We are planning to release to other package managers in the future once floo becomes more stable.*

Or alternatively, you may choose to build from source:

```sh
git clone 'https://github.com/keyvizsla/floo.git'
cd floo
cargo install --path .
```

Afterwards, you can verify that this first installation step was successful by verifying that the following prints `floo x.x.x`, depending on the current version:

```sh
floo-bin --version
```

You are now ready to initialize your shell environment to integrate with *floo*.
Please follow the instructions listed below for the shell that you are using to finish your setup.

## Bash

Run the following command:

```bash
echo 'eval "$(floo-bin init)"' >> ~/.bashrc
```

And restart your terminal.

## Zsh

Run the following command:

```bash
echo 'eval "$(floo-bin init)"' >> ~/.zshrc
```

And restart your terminal.

## Nushell

Run the following command:

```nu
floo-bin init --shell nu | save $nu.config-path --append
```

And restart your terminal.

*Note: floo was initially developed with bash in mind. Due to the fundamentally different paradigms of nushell, we havent been able to port its full functionality to this shell, yet. In nushell, you cannot create custom .floo scripts to define custom actions to take when travelling to a fireplace. In nushell, floo will always perform the default action of bringing you into the selected fireplace's project directory and sourcing any .env/.envrc files in that directory.*

## Other Shells

If your favorite shell is missing from this list, we are sorry to let you know that *floo* does not yet support it.
Please raise an issue on [the GitHub repository](https://github.com/keyvizsla/floo/issues) so that we know which
shell you would like to use *floo* in and we will try and accomodate you.
