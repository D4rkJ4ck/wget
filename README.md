<h1 align=center>
    <img alt="Ferris" src="./ferris.svg">
    <br>
    wget
</h1>

## Table of Contents
- [Overview](#overview)
- [Tech Stack](#tech-stack)
- [Installation](#installation)
    - [Cloning](#cloning)
    - [File Structure](#file-structure)
- [Usage](#usage)
- [Contributors](#contributors)
- [License](#license)

## Overview

Wget is a free utility for non-interactive download of files from the Web. It supports HTTP, HTTPS, and FTP protocols, as well as retrieval through HTTP proxies.

To see more about wget you can visit the manual by using the command man wget, or you can visit the website [here](https://www.gnu.org/software/wget/manual/wget.html).

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
    |       |       + file_name.rs
    |       |       + file_path.rs
    |       |       + multi_links.rs
    |       |       + mod.rs
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

## Usage

## Contributors

## License

