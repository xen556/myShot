# myshot

**myshot** is a lightweight and fast tool for selecting a screen area and taking screenshots on Linux.
**Requires [`slurp`](https://github.com/emersion/slurp) to select a screen region.**
The program comes as a precompiled binary; no compilation is needed.

---

## Features

* Select a specific area of the screen (`--area`) using `slurp`
* Capture the entire screen (`--fullscreen`)
* Simple and minimal interface

---

## Installation

Download the precompiled binary and place it in a directory in your `PATH`, or run it directly:

```bash
./myshot --help
```

To make it available system-wide:

```bash
sudo cp myshot /usr/local/bin/
```

---

## Requirements

* Linux with a graphical environment
* [`slurp`](https://github.com/emersion/slurp) installed

Install `slurp` on common Linux systems:

```bash
# Debian/Ubuntu
sudo apt install slurp

# Arch/Manjaro
sudo pacman -S slurp

# NixOS
nix-env -iA nixpkgs.slurp
```

---

## Usage

```bash
./myshot --help
```

Output:

```
Usage: myshot [OPTIONS]

Options:
      --area        Select a specific area of the screen (requires slurp)
      --fullscreen  Capture the entire screen
  -h, --help        Print help information
```

### Examples

Capture a specific area:

```bash
./myshot --area
```

Capture the full screen:

```bash
./myshot --fullscreen
```

---

## Notes

* No compilation is required; the binary is ready to use.
* Future versions may add support for macOS and Windows.

---
