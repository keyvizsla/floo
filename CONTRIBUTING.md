# Contributing to Floo

Hey there, thanks four your interest in becoming involved with the development of *floo*.
We will help you with any technical issues and help you improve your contribution if at all necessary.

## How to Contribute

For now, we operate on a system of mutual trust and good judgment.
Please open issues with bugs you have found or features you feel are missing from the application.
Just make sure to provide some context on how we would be able to reproduce the bug.
If you find something that you know how to fix or you proactively implemented a feature you were missing,
please go ahead and submit a PR!

You can contribute in many different ways. You can either open an issue on GitHub describing a bug you
have encountered or a feature that *floo* is missing to suit your needs.
We don't expect you to be an expert, so don't worry if you are unsure or don't know how to actually
implement the feature you are describing, we can figure things out together.

### Contributing Code to *floo*

Code contributions should be straightforward to you if you have done this kind of thing before.
If you don't yet quite know where to start, here is a good guide on how you can forward your code to us:
[GitHub guide](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/working-with-forks/fork-a-repo)

When opening a pull-request, please make sure that you either link to an existing issue or provide an
adequate description of what feature your pull-request implements, what issue it solves and which
parts you would like further guidance on, should you need any.

### Testing

Please make sure that you add tests for new features that you have implemented or a regression test
for the bug that you have fixed. *floo* uses 3 types of tests to verify its correct behavior:

**Unit tests**: These are fairly straightforward, written in the rust source files themselves and you should add them if your code touches some standalone function that doesn't directly depend on the TUI itself.

**Snapshot tests**: These act to ensure that there are no unexpected changes to the TUI. The [ratatui docs](https://ratatui.rs/recipes/testing/snapshots/) provide a nice introduction to these types of tests. To see an example of how these tests are implemented in *floo* you can take inspiration from the `start_screen` component.

To run the current suite of unit/snapshot tests you can use the following command from the root of the *floo* repository:

```sh
docker run --rm -v $(pwd):/app ghcr.io/keyvizsla/floo-linux:latest
```

**Integration / E2E tests**: These check that the full chain from the user perspective of spinning up *floo* and being transportet into the correct shell state does indeed work. You generally won't have to write or alter these kinds of tests. Adding new tests to this domain is only necessary for major changes like adding support for a new shell type. We will let you know should your changes prompt the creation of such a test and assist you in writing those tests, should you need any help.

To run the end-to-end tests use the following command:
```sh
docker run --rm -v $(pwd):/app ghcr.io/keyvizsla/floo-linux:latest bash -c " \
  cargo install --path .; \
  python tests/e2e_test_base.py;
"
```


