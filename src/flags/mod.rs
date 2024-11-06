//! The normal usage of wget: downloading a file given an URL, example:
//! ```
//! wget https://some_url.ogr/file.zip
//! ```
//!
//! Additional Functionalities:
//!     + Downloading a single file and saving it under a different name
//!     + Downloading and saving the file in a specific directory
//!     + Set the download speed, limiting the rate speed of a download
//!     + Downloading a file in background
//!     + Downloading multiple files at the same time, by reading a file containing multiple download links asynchronously
//!     + Main feature will be to download an entire website, mirroring a website [https://en.wikipedia.org/wiki/Mirror_site]

mod background;
mod links;
mod mirror;
mod name;
mod path;
mod rate_limit;
