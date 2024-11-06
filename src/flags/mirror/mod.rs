//! Mirror a website. This option should download the entire website being possible to use "part" of the website offline and for other useful reasons [https://www.quora.com/How-exactly-does-Mirror-Site-works-and-how-it-is-done].
//! For this you will have to download the website file system and save it into a folder that will have the domain name.
//! Example: [http://www.example.com], will be stored in a folder with the name `www.example.com` containing every file from the mirrored website.
//! The flag should be `--mirror`.
//!
//! The default usage of the flag will be to retrieve and parse the HTML or CSS from the given URL. This way retrieving the files that the document refers through tags.
//! The tags that will be used for this retrieval must be a, `link` and `img` that contains attributes `href` and `src`.
//!
//! You will have to implement some optional flags to go along with the `--mirror` flag.
//!
//! Those flags will work based on Follow links [https://www.gnu.org/software/wget/manual/wget.html#Following-Links].
//! The command `wget` has several mechanisms that allows you to fine-tune which links it will follow.
//! For This project you will have to implement the behavior of (note that this flags will be used in conjunction with the `--mirror` flag):

mod convert_links;
mod exclude;
mod reject;
