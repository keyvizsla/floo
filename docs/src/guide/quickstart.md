# Quickstart

Having installed **floo** on your system, you are now ready to build your floo network.
In **floo** you are able to define fireplaces. Fireplaces equate to projects or workspaces
to which you want to be able to travel to and from quickly, with optional additional configurations
about what state you want your shell to be brought to once you reach your destination.

## Basic setup

When you first run `floo` in your terminal, you should see this screen:
![A screenshot of the floo startscreen with no fireplaces](../assets/startscreen.png)

As prompted, you can now get started setting up your first fireplace by pressing one of `n` or `%`.
Most of the keybinds in **floo** should feel natural to you if you have been working with other
TUI applications, you are however always welcome to provide alternative suggestions as an issue.

![A screenshot of the initial fireplace creation dialog](../assets/creation_popup.png)

Using `tab` or your arrow keys, navigate through the dialog, filling in the blanks, to create your
first fireplace. For the path to the fireplace, you should put the path to the root directory
of your project, for example for projects using git this would be the directory containing the .git
directory.
For your convenience, **floo** supports relative paths to where it is currently being executed,
that means if you ran `floo` from the root of the directory in which you want to have a fireplace,
you can just put `.` as path to fireplace. On future executions, **floo** will correctly resolve
the path regardless of which directory you call **floo** from.
Alternatively, if you already are in the directory where you want to create a fireplace,
you can also run `floo create` which will start floo with a prefilled fireplace creation dialog,
so you don't have to manually insert your path.

You have now successfully created your first fireplace and are all set to travel.
If you want to know how to set up custom actions for your fireplace, templating and
general usage tips, consult the *User Manual* section of this guide.

## Troubleshooting

We will keep updating this section with common issues users run into and their fixes.
Please help us in improving this documentation by raising an issue where you describe
your troubles.
Alternatively, you can send an email to [leon.koehn2002@gmail.com](mailto:leon.koehn2002@gmail.com?subject=Issues%20regarding%20floo).
We will get back to you as soon as possible.
If you were able to resolve your troubles, even if you think no one else would make that mistake,
we always appreciate PRs updating this troubleshooting guide.
