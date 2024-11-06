//! Directory-Based Limits [https://www.gnu.org/software/wget/manual/wget.html#Directory_002dBased-Limits] (--exclude short hand -X)
//!
//! this flag will have a list of paths that the program will avoid to follow and retrieve.
//! So if the URL is https://example.com and the directories are /js, /css and /assets you can avoid any path by using -X=/js,/assets.
//! The fs will now just have /css.
//!
//! example:
//! ```
//! $ cargo r --mirror -X=/assets,/css https://example.com
//! ```
