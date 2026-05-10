# Installation

## Installing via curl

Installing the latest release is as easy as running the following in your terminal:

```sh
curl --proto '=https' --tlsv1.2 -sSfL https://raw.githubusercontent.com/keyvizsla/floo/main/scripts/install.sh | sh
```

Afterwards, you should restart your terminal or source your `.bashrc` or `.zshrc` for the changes
to take effect. You may check your installation by running `floo --version`. If you successfully
get back the current floo version, you are ready to proceed to the *Quickstart* section to get
started integrating floo into your workflow.

## Installing from source

Installing from source requires two seperate steps:

1. Building the actual floo-bin binary
2. Adding a shell hook for floo

The release installer does both of these for you, if you are installing from source,
you should perform these steps yourself.
Step 1 is straightforward if you have a working toolchain, to build and run projects using [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html).
To be able to obtain the source code you should also have `git` installed on your system.
If your system meets the above criteria, building the floo binary is as simple as running the following:

```sh
git clone https://github.com/keyvizsla/floo.git
cd floo
cargo build --release
```

For step 2, add the path to `floo-bin` e.g. `target/release/floo-bin` to your `PATH`.
You may choose to place `floo-bin` into a directory that is likely already on `PATH` like
`/usr/bin`.
Check your setup by running `floo-bin --version`, this should successfully print a line
containing the latest floo version.
Once this works, you now only have to add the following line to your `.bashrc` or `.zshrc`
respectively:

```sh
eval "$(floo-bin init)"
```

You may now restart your terminal or simply source aforementioned rc-file.
You should now be able to run `floo`, upon which the TUI should open.

## Installing via homebrew

We plan to provide homebrew releases for floo.
This is still a work in progress, for the time being, please consider installing
floo via the curl installer or building from source.

