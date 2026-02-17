# rShot

**rShot** is a lightweight and fast tool for taking screenshots on Linux, either fullscreen or from a selected screen area.

**Requires:**  

* [`slurp`](https://github.com/emersion/slurp) – to select a screen region  
* [`wl-clipboard`](https://github.com/bugaevc/wl-clipboard) – to copy screenshots to the system clipboard on Wayland  

---

## Features

* Select a specific area of the screen (`--area`) using `slurp`
* Capture the entire screen (`--fullscreen`)
* Automatically copy screenshots to the system clipboard (requires `wl-copy`)
* Simple and minimal interface

---

## Installation

Download the precompiled binary and place it in a directory in your `PATH`, or run it directly:

--- 

## Requirements

* Linux with a graphical environment(Wayland)
* [`slurp`](https://github.com/emersion/slurp) installed
* wl-clipboard installed

--- 

Install `slurp and wl-clipboard` on common Linux systems:

```bash
# Debian/Ubuntu
sudo apt install slurp wl-clipboard

# Arch/Manjaro
sudo pacman -S slurp wl-clipboard

# NixOS
nix-env -iA nixpkgs.slurp nixpkgs.wl-clipboard
```

---

## Usage

```bash
./rshot --help
```

Output:

```
Usage: rshot [OPTIONS]

Options:
      --area        Select a specific area of the screen (requires slurp)
      --fullscreen  Capture the entire screen
  -h, --help        Print help information
```

## Examples

Capture a specific area:

```bash
./rshot --area
```

Capture the full screen:

```bash
./rshot --fullscreen
```

---

## Notes 
* Screenshots are saved to `~/Pictures/Screenshots` by default.
