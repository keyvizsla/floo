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

![](/assets/tmux_example.gif)
*An example usage of the default tmux template*

## Creating new templates

Creating a new template happens in two steps:

1. Write and test the .floo script
2. Declare the script as a template

The first step we have outlined in the [quickstart](/guide/quickstart.html#defining-custom-actions) section of this guide.
For the second step you simply run

```sh
floo template [path] [name]
```

where the path is the filepath of the .floo script you want to turn into a template and
the name is the name under which floo will list your new template in all future template 
selection screens.

## Using floo variables

To keep templates generic, you can make use of specific environment variables, which
`floo` guarantees to you to be set every time before a .floo script is executed.
This applies to the following variables:

| Variable | Description |
|---------------|-------------|
| FLOO_DIR | The absolute path of the fireplace's configured directory|
| FLOO_NAME | The name of the fireplace |

Check out the default tmux template to see how to use these in practice.
