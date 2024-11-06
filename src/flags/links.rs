//! Downloading different files should be possible.
//! For this the program will receive the `-i` flag followed by a file name that will contain all links that are to be downloaded.
//! Example:
//! ```
//! $ ls
//! download.txt   main.go
//!
//! $ cat download.txt
//! http://ipv4.download.thinkbroadband.com/20MB.zip
//! http://ipv4.download.thinkbroadband.com/10MB.zip
//!
//! $ cargo r -i=download.txt
//! content size: [10485760, 20971520]
//! finished 10MB.zip
//! finished 20MB.zip
//!
//! Download finished:  [http://ipv4.download.thinkbroadband.com/20MB.zip http://ipv4.download.thinkbroadband.com/10MB.zip]
//!
//! $
//! ```
//! The Downloads should work asynchronously, it should download both files at the same time.
//! You are free to display what you want for this option.
