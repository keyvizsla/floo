# Introduction

<style>
    .floo-version {
        position: absolute;
        right: 20px;
        top: 60px;
        background-color: var(--theme-popup-bg);
        border-radius: 8px;
        padding: 2px 5px 2px 5px;
        border: 1px solid var(--theme-popup-border);
        font-size: 0.9em;
    }
</style>

<div class="floo-version">
Version: {{ floo_version }}
</div>

**floo** is a terminal user interface (TUI) to automate workspace setup and access.

If you have a very terminal-centric workflow, you oftentimes have to perform a set of repetitive
tasks each time you fire up your machine, before you can start being productive.
For me this looks something like the following:

1. cd into the root directory of your project
2. start a tmux session with multiple windows
3. source workspace specific environment configurations (e.g. virtualenv, or just env stuff) in each window
4. run your favorite editor in one window
4. open a webbrowser with some documentation or github issues you are browsing
5. start actually working on your stuff

**floo** relieves you of all this manual overhead just to jump into a project.
Inspired by the floo network from the Harry Potter universe, you can dynamically add or
remove fireplaces (project/workspace configurations) to it.
Setting up a new fireplace is as easy as pressing `n` in the TUI and entering a name and root directory
of your workspace. You only have to set up a fireplace once, **floo** will remember it for you afterwards.
Once a fireplace is set up, you can simply run `floo`, select the desired fireplace in the TUI
and your shell will be brought into the configured state. For me that typically is a new tmux session, with two windows and
all environment configurations sourced and some small project specific adjustments.
Configuring the concrete actions you want to have performed is as easy as placing a shell script named `.floo`
into the corresponding project directory. **floo** will make sure the shell sources this script before
passing control back to you, such that all modifications you have defined in your script are applied.
**floo** assists you in setting up these scripts for your fireplaces by providing you with templates that
you can modify and apply with ease.
You may even add your own templates aligning with your concrete needs, to streamline setting up new fireplaces with your specific workflow.

## A first look

![](assets/basic_example.gif)
*Basic usage example of floo, we create a new fireplace which we then use to quicktravel into that directory, any .env or .envrc files of that directory are sources as well.*

![](assets/tmux_example.gif)
*More advanced usage example, we use a predefined template to define custom floo actions, to let floo know to start a tmux session with two windows when travelling to the fireplace.*

![](assets/notes_example.gif)
*For convenience, floo allows you to also track notes for each fireplace, this supports markdown and can help you track where you left off or whatever else you want to remember.*

## License

Copyright (C) 2026 Leon Degel-Koehn

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
