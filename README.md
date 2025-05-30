<h1 align=center>
    <img alt="Ferris" src="./public//ferris.svg">
    <br>
    wget
</h1>

## Table of Contents

- [Table of Contents](#table-of-contents)
- [Overview](#overview)
        - [_Table of Contents ⤴️_](#table-of-contents-️)
- [Tech Stack](#tech-stack)
- [Installation](#installation)
  - [Cloning](#cloning)
  - [File Structure](#file-structure)
        - [_Table of Contents ⤴️_](#table-of-contents-️-1)
- [Architecture](#architecture)
  - [Entities](#entities)
  - [Sequence](#sequence)
- [Usage](#usage)
  - [Background Download](#background-download)
  - [Custom Name](#custom-name)
  - [Custom Path](#custom-path)
  - [Rate Limit](#rate-limit)
  - [Different Files](#different-files)
        - [_Table of Contents ⤴️_](#table-of-contents-️-2)
  - [Mirror a website](#mirror-a-website)
    - [Types of Files(--reject short hand -R)](#types-of-files--reject-short-hand--r)
    - [Directory-Based Limits (--exclude short hand -X)](#directory-based-limits---exclude-short-hand--x)
    - [Convert Links for Offline Viewing (--convert-links)](#convert-links-for-offline-viewing---convert-links)
        - [_Table of Contents ⤴️_](#table-of-contents-️-3)
- [Contributors](#contributors)
  - [Collaborators](#collaborators)
  - [Peers](#peers)
- [License](#license)
        - [_Table of Contents ⤴️_](#table-of-contents-️-4)

## Overview

`WGET` is a free utility for non-interactive download of files from the Web. It supports `HTTP`, `HTTPS`, and `FTP` protocols, as well as retrieval through `HTTP proxies`.  
To see more about wget you can visit the manual by using the command man wget, or you can visit the website [here](https://www.gnu.org/software/wget/manual/wget.html).

The program should be able to give feedback, displaying the:

- Time that the program started: it must have the following format `yyyy-mm-dd hh:mm:ss`
- Status of the request. For the program to proceed to the download, it must present a response to the request as status OK (**200 OK**) if not, it should say which status it got and finish the operation with an error warning
- Size of the content downloaded: the content length can be presented as raw (**bytes**) and rounded to `Mb` or `Gb` depending on the size of the file downloaded
- Name and path of the file that is about to be saved
- A progress bar, having the following:
  - A amount of `KiB` or `MiB` (depending on the download size) that was downloaded
  - A percentage of how much was downloaded
  - Time that remains to finish the download
- Time that the download finished respecting the previous format

It should look something like this

```shell
cargo r https://pbs.twimg.com/media/EMtmPFLWkAA8CIS.jpg
start at 2017-10-14 03:46:06
sending request, awaiting response... status 200 OK
content size: 56370 [~0.06MB]
saving file to: ./EMtmPFLWkAA8CIS.jpg
 55.05 KiB / 55.05 KiB [==============================================================] 100.00% 1.24 MiB/s 0s
Downloaded [https://pbs.twimg.com/media/EMtmPFLWkAA8CIS.jpg]
finished at 2017-10-14 03:46:07
```

###### [_Table of Contents ⤴️_](#table-of-contents)

## Tech Stack

[![RUST](https://img.shields.io/badge/Rust-black?style=for-the-badge&logo=rust&logoColor=#E57324)](./src/main.rs)
[![SHELL SCRIPT](https://img.shields.io/badge/Shell_Script-121011?style=for-the-badge&logo=gnu-bash&logoColor=white)](./gitify.sh)
[![MARKDOWN](https://img.shields.io/badge/Markdown-000000?style=for-the-badge&logo=markdown&logoColor=white)](#table-of-contents)
[![GITHUB](https://img.shields.io/badge/GitHub-100000?style=for-the-badge&logo=github&logoColor=white)](https://github.com/D4rkJvck/smart-road.git)
[![WARP](https://img.shields.io/badge/warp-01A4FF?style=for-the-badge&logo=warp&logoColor=white)]()
[![LINUX](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)]()
[![MAC OS](https://img.shields.io/badge/mac%20os-000000?style=for-the-badge&logo=apple&logoColor=white)]()

## Installation

### Cloning

To clone this repository, you can use the following command:

```shell
git clone http://learn.zone01dakar.sn/git/jefaye/wget
cd wget
tree --dirsfirst
```

### File Structure

```
📂./
  |
  +-📂 /scripts
  |     |
  |     +-📜 gitify.sh
  |
  +-📂 /src
  |     |
  |     +-📂 /flags
  |     |     |
  |     |     +-📂 /mirror
  |     |     |     |
  |     |     |     +-📄 convert_links.rs
  |     |     |     +-📄 exclude.rs
  |     |     |     +-📄 mod.rs
  |     |     |     +-📄 reject.rs
  |     |     |
  |     |     +-📄 background.rs
  |     |     +-📄 name.rs
  |     |     +-📄 path.rs
  |     |     +-📄 multi.rs
  |     |     +-📄 mod.rs
  |     |     +-📄 rate_limit.rs
  |     |
  |     +-📄 lib.rs
  |     +-📄 main.rs
  |
  +-📂 /tests
  |
  +-📂 /todo
  |     |
  |     +-📝 audit.todo
  |     +-📝 tasks.todo
  |
  +-🚫 .gitignore
  +-🔒 Cargo.lock
  +-⚙️ Cargo.toml
  +-🏞 ferris.svg
  +-🔑 LICENSE
  +-📖 README.md
```

###### [_Table of Contents ⤴️_](#table-of-contents)

## Architecture

```mermaid
architecture-beta
  group wget(logos:rust)[wget]
  group src(logos:rust)[source] in wget
  group pub(disk)[public] in wget

  service args(logos:bash-icon)[args]
  service parser(logos:blocs)[parser] in src
  service downloader(logos:firefox)[downloader] in src
  service output(logos:google-keep)[ouput] in pub


  args:R --> L:parser
  parser:B --> T:downloader
  downloader:R --> L:output
```

### Entities

```mermaid
classDiagram
  class Args {
    <<struct>>
    +background
    +output
    +path
    +rate_limit
    +input
    +mirror
    +reject
    +exclude
    +convert_links
    +url
  }
  Parser ()-- Args
  Debug ()-- Args

  class Downloader {
    <<struct>>
    +download(args)
  }

  Args *-- Downloader: 

```

### Sequence

```mermaid
sequenceDiagram
  participant Args
  participant Downloader

  Args ->> Args: Parse
  Args ->> Downloader: Passed to
  Downloader ->> Downloader: Get 
```

## Usage

The normal usage of wget: downloading a file given an URL, example:

```shell
wget https://some_url.ogr/file.zip
```

Additional Functionalities:

- [Downloading a file in `background`](#background-download)
- [Downloading a single file and saving it under a different `name`](#custom-name)
- [Downloading and saving the file in a `specific directory`](#custom-path)
- [Set the download speed, `limiting the rate speed` of a download](#rate-limit)
- [Downloading `multiple files` at the same time, by reading a `file` containing multiple `download links asynchronously`](#different-files)
- [Main feature will be to `download an entire website`](#mirror-a-website)

### Background Download

The flag `-B` should be handled, this flag should download a file immediately to the background and the output should be redirected to a log file.
When the program containing this flag is executed it should output : `Output will be written to "wget-log"`.
Example:

```shell
cargo r -B https://pbs.twimg.com/media/EMtmPFLWkAA8CIS.jpg
Output will be written to "wget-log".

cat wget-log
start at 2017-10-14 03:46:06
sending request, awaiting response... status 200 OK
content size: 56370 [~0.06MB]
saving file to: ./EMtmPFLWkAA8CIS.jpg
Downloaded [https://pbs.twimg.com/media/EMtmPFLWkAA8CIS.jpg]
finished at 2017-10-14 03:46:07
```

### Custom Name

Download a file and save it under a different name by using the flag `-O` followed by the name you wish to save the file, example:

```shell
cargo r -O=meme.jpg https://pbs.twimg.com/media/EMtmPFLWkAA8CIS.jpg
start at 2017-10-14 03:46:06
sending request, awaiting response... status 200 OK
content size: 56370 [~0.06MB]
saving file to: ./meme.jpg
 55.05 KiB / 55.05 KiB [==============================================================] 100.00% 1.24 MiB/s 0s
Downloaded [https://pbs.twimg.com/media/EMtmPFLWkAA8CIS.jpg]
finished at 2017-10-14 03:46:07

ls -l
-rw-r--r-- 1 student student 56370 ago 13 16:59 meme.jpg
-rw-r--r-- 1 student student 11489 ago 13 10:28 main.go
```

### Custom Path

It should also handle the path to where your file is going to be saved using the flag `-P` followed by the path to where you want to save the file, example:

```shell
cargo r -P=~/Downloads/ -O=meme.jpg https://pbs.twimg.com/media/EMtmPFLWkAA8CIS.jpg
start at 2017-10-14 03:46:06
sending request, awaiting response... status 200 OK
content size: 56370 [~0.06MB]
saving file to: ~/Downloads/meme.jpg
 55.05 KiB / 55.05 KiB [==============================================================] 100.00% 1.24 MiB/s 0s
Downloaded [https://pbs.twimg.com/media/EMtmPFLWkAA8CIS.jpg]
finished at 2017-10-14 03:46:07

ls -l ~/Downloads/meme.jpg
-rw-r--r-- 1 student student 56370 ago 13 16:59 /home/student/Downloads/meme.jpg
```

### Rate Limit

The program should handle speed limit.
Basically the program can **control the speed** of the download by using the flag `--rate-limit`.
If you download a huge file you can limit the speed of your download, preventing the program from using the full possible **bandwidth** of your connection, example:

```shell
cargo r --rate-limit=400k https://pbs.twimg.com/media/EMtmPFLWkAA8CIS.jpg
```

This flag should accept different value types, example: `k` and `M`. So you can put the rate limit as `rate-limit=200k` or `rate-limit=2M`

### Different Files

Downloading **different files** should be possible.
For this the program will receive the `-i` flag followed by a file name that will contain all links that are to be downloaded.
Example:

```shell
ls
download.txt   main.go

cat download.txt
http://ipv4.download.thinkbroadband.com/20MB.zip
http://ipv4.download.thinkbroadband.com/10MB.zip

cargo r -i=download.txt
content size: [10485760, 20971520]
finished 10MB.zip
finished 20MB.zip
Download finished:  [http://ipv4.download.thinkbroadband.com/20MB.zip http://ipv4.download.thinkbroadband.com/10MB.zip]
```

The Downloads should work **asynchronously**, it should download both files at the same time.
You are free to display what you want for this option.

###### [_Table of Contents ⤴️_](#table-of-contents)

### [Mirror a website](https://en.wikipedia.org/wiki/Mirror_site)

Mirror a website. This option should download the entire website being possible to use **"part"** of the website offline and for other useful [reasons](https://www.quora.com/How-exactly-does-Mirror-Site-works-and-how-it-is-done).
For this you will have to download the website file system and save it into a folder that will have the domain name.
Example: [http://www.example.com], will be stored in a folder with the name **www.example.com** containing every file from the mirrored website.
The flag should be `--mirror`.  
The default usage of the flag will be to retrieve and parse the HTML or CSS from the given URL. This way retrieving the files that the document refers through tags.
The tags that will be used for this retrieval must be a, `link` and `img` that contains attributes `href` and `src`.
You will have to implement some optional flags to go along with the `--mirror` flag.  
Those flags will work based on [Follow links](https://www.gnu.org/software/wget/manual/wget.html#Following-Links).
The command `wget` has several mechanisms that allows you to fine-tune which links it will follow.
For This project you will have to implement the behavior of (note that this flags will be used in conjunction with the `--mirror` flag):

#### [Types of Files](https://www.gnu.org/software/wget/manual/wget.html#Types-of-Files)(--reject short hand -R)

this flag will have a list of file suffixes that the program will avoid downloading during the retrieval
example:

```shell
cargo r --mirror -R=jpg,gif https://example.com
```

#### [Directory-Based Limits](https://www.gnu.org/software/wget/manual/wget.html#Directory_002dBased-Limits) (--exclude short hand -X)

this flag will have a list of paths that the program will avoid to follow and retrieve.
So if the URL is **https://example.com** and the directories are /js, /css and /assets you can avoid any path by using -X=/js,/assets.
The fs will now just have /css.
example:

```shell
cargo r --mirror -X=/assets,/css https://example.com
```

#### [Convert Links for Offline Viewing](https://www.gnu.org/software/wget/manual/wget.html#The-%60--convert_002dlinks%60-Option) (--convert-links)

this flag will convert the links in the downloaded files so that they can be viewed offline, changing them to point to the locally downloaded resources instead of the original URLs.
example:

```shell
cargo r --mirror --convert-links https://example.com
```

###### [_Table of Contents ⤴️_](#table-of-contents)

## Contributors

### Collaborators

[![ndiediop](https://shields.io/badge/ndiediop-Zone01-magenta)](http://learn.zone01dakar.sn/git/ndiediop)
[![fakeita](https://shields.io/badge/fakeita-Zone01-magenta)](http://learn.zone01dakar.sn/git/fakeita)
[![abdoulaziba](https://shields.io/badge/abdoulaziba-Zone01-cyan)](http://learn.zone01dakar.sn/git/abdoulaziba)
[![jefaye](https://shields.io/badge/jefaye-Zone01-cyan)](http://learn.zone01dakar.sn/git/jefaye)

### Peers

[![npouille](https://shields.io/badge/npouille-Zone01-blue)](http://learn.zone01dakar.sn/git/npouille)
[![eibounda](https://shields.io/badge/eibounda-Zone01-red)](http://learn.zone01dakar.sn/git/eibounda)

## License

[![MIT](https://shields.io/badge/License-MIT-yellow)](LICENSE)

###### [_Table of Contents ⤴️_](#table-of-contents)
