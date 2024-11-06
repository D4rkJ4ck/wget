<h1 align=center>
    <img alt="Ferris" src="./ferris.svg">
    <br>
    wget
</h1>

## Table of Contents
- [Overview]()
- [Tech Stack]()
- [Installation]()
    - [Cloning]()
    - [File Structure]()
    - [Blueprints]()
- [Usage]()
- [Contributors]()
- [License]()

## Overview

wget is a command line tool that allows users to download files from the internet.

## Tech Stack

## Installation

### Cloning

To clone this repository, you can use the following command:
```shell
git clone http://learn.zone01dakar.sn/git/jefaye/wget
cd wget
tree --dirsfirst
```

### File Structure
    .
    |
    +----- src/
    |       |
    |       +--- flags/
    |       |       |
    |       |       +-- mirror/
    |       |       |       |
    |       |       |       + convert_links.rs
    |       |       |       + exclude.rs
    |       |       |       + mod.rs
    |       |       |       + reject.rs
    |       |       |
    |       |       + background.rs
    |       |       + links.rs
    |       |       + mod.rs
    |       |       + name.rs
    |       |       + path.rs
    |       |       + rate_limit.rs
    |       |
    |       + lib.rs
    |       + main.rs
    |
    +---- todo/
    |       |
    |       + audit.todo
    |       + tasks.todo
    |
    + .gitignore
    + Cargo.lock
    + Cargo.toml
    + ferris.svg
    + gitify.sh
    + LICENSE
    + README.md
