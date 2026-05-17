# floo

<p align="center">
  <img src="assets/logo_small.png" /><br/>
  <img src="https://github.com/keyvizsla/floo/actions/workflows/deploy_pages.yaml/badge.svg" />
</p>

## What is floo

Inspired by the concept of the floo network of a popular fantasy franchise,
**floo** provides you with a central interface to quickly travel between
workspaces and projects, relieving you of manual navigation and setup of
environments in your terminal.

## How it works

At its core, **floo** allows you to dynamically add and remove fireplaces
from your network. A fireplace has a name and a directory it is connected to.
Via the tui of **floo** you can select a fireplace to travel to, upon which
**floo** transports you right to that workspace, bringing your terminal into
the exact state you need it in for that project. This means **floo** can
set environment variables, open tmux sessions, editors, browsers, you name it,
whatever you needed to manually do to get going on a project, **floo** can take
care of for you.

## Resources

[Manual](https://keyvizsla.github.io/floo/index.html)

## Installation

## Known Limitations

- When applying a template, it is applied regardless whether or not the user has saved their modifications to the template. That is, if they just quit the editor without saving, the default template is applied, whereas it would be preferrable to instead not apply anything at all. If this happens to you and you did not intend to apply the template, simply run `rm .floo` in the fireplace's directory.

## News

<Meaningful usage gif highlighting most important feature>

<Maybe getting started guide or just a pages reference>

<Contributing>

<Feedback?>

<License>
