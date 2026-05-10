# Working with templates

Templates allow you to re-use `.floo` configurations for different workspaces.
Going back to the running example of a terminal-based tmux workflow, regardless
whether or not you want to work on a python project, a rust project, a latex project or
whatever it may be, you would likely want a similar setup each time:
Create a named tmux session that has a certain number of open windows, where each window
starts running a certain process automatically.
A template for this sort of workflow comes pre-packaged with floo.
Floo will tell you, when a fireplace has no `.floo` configuration, yet.
By pressing `e` in the info view, you can select from the list of templates installed on
your system to get started from. Once you have selected a template, `floo` will open your
systems `$EDITOR`, where you can then tweak the exact contents of the template for the
fireplace you are setting up.

TODO: Insert gif of choosing and editing a template

## Creating new templates

TODO Write docs how to create a template

TODO Insert gif of how this looks in practice


