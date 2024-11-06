//! Convert Links for Offline Viewing [https://www.gnu.org/software/wget/manual/wget.html#The-%60--convert_002dlinks%60-Option] (--convert-links)
//!
//! this flag will convert the links in the downloaded files so that they can be viewed offline, changing them to point to the locally downloaded resources instead of the original URLs.
//!
//! example:
//! ```
//! $ cargo r --mirror --convert-links https://example.com
//! ```
