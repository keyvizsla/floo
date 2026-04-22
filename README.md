# floo

**Ignite your workspace. Travel between projects at terminal velocity.**

![](assets/logo.png)

[](https://opensource.org/licenses/MIT)
[](https://ratatui.rs)
[](https://www.rust-lang.org)

-----

## ⚡️ What is floo?

**floo** is a TUI (Terminal User Interface) project manager designed for developers who live in the terminal. If your morning routine involves a tedious sequence of `cd`, `tmux new-session`, `source .env`, and opening `nvim`, floo is your magic powder.

Inspired by the Harry Potter universe, floo treats your project directories as **Fireplaces**. With a single command, you can "ignite" a fireplace, instantly transporting your terminal session to the project root and executing custom startup rituals.

[Insert GIF of floo in action: selecting a project and watching tmux/nvim launch]

-----

## ✨ Features

  * **⚡ Rapid Navigation:** Scroll or fuzzy-search through your "Fireplaces" (projects) and jump to them instantly.
  * **🔥 Workspace Ignition:** Automatically detects and sources a `.floo` script in your project directory upon entry.
  * **🛠 Environment Automation:** Perfect for setting up tmux sessions, gcloud logins, or docker-compose environments.
  * **📦 Light & Fast:** Built with Rust and Ratatui for a footprint-free, high-performance experience.
  * **🧙 Minimalist Design:** No complex configuration files; just point, click (or keyboard), and travel.

-----

## 🚀 Getting Started

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/floo.git

# Move into the directory
cd floo

# Build and install
cargo install --path .
```

### Setup

To allow **floo** to change your shell's current working directory, you may need to add a small alias to your `.zshrc` or `.bashrc`:

```bash
alias floo='source _floo_wrapper'
```

-----

## 📖 How it Works

### 1\. Register a Fireplace

Open `floo` and add a new project. Give it a name and point it to your project's directory.

### 2\. Prepare the Floo Powder

Create a `.floo` file in your project root. This is a standard shell script that defines your ideal environment.

**Example `.floo` script:**

```bash
#!/bin/bash
# Set up a tmux session with two windows
tmux new-session -d -s "my-project"
tmux rename-window -t 0 'editor'
tmux send-keys -t 0 'nvim .' C-m
tmux new-window -t 1 -n 'terminal'
tmux select-window -t 0
tmux attach-session -t "my-project"
```

### 3\. Ignite

Simply run `floo`, select your project, and watch as your directory changes and your environment spawns instantly.

-----

## 🎨 Interface

| View | Description |
| :--- | :--- |
| **Main Dashboard** | A list of all registered Fireplaces and their paths. |
| **Search/Filter** | Quickly narrow down projects by name. |
| **Add/Edit** | Simple forms to manage your project list. |

[Insert Screenshot of the Main TUI Dashboard here]

-----

## 🛠 Tech Stack

  * **Language:** [Rust](https://www.rust-lang.org/)
  * **TUI Framework:** [Ratatui](https://github.com/ratatui-org/ratatui)
  * **Backend:** [Crossterm](https://github.com/crossterm-rs/crossterm)

-----

## 🤝 Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

1.  Fork the Project
2.  Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3.  Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4.  Push to the Branch (`git push origin feature/AmazingFeature`)
5.  Open a Pull Request

-----

## 📜 License

Distributed under the MIT License. See `LICENSE` for more information.

-----

\<p align="center"\>
Built with ❤️ for the terminal-centric developer.
\</p\>

### Future features

- Sorting: Users can configure the order in which to list their fireplaces (by last used, lexicographically, ...)
- Auto-Ignition: Templates for `.floo` script creation. Workflows oftentimes look very similar.
- Add git status of fireplace in tui overview, beyond the README.
