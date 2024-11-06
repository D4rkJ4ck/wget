//! The program should be able to give feedback, displaying the:
//!
//!     + Time that the program started: it must have the following format yyyy-mm-dd hh:mm:ss
//!     + Status of the request. For the program to proceed to the download, it must present a response to the request as status OK (200 OK) if not, it should say which status it got and finish the operation with an error warning
//!     + Size of the content downloaded: the content length can be presented as raw (bytes) and rounded to Mb or Gb depending on the size of the file downloaded
//!     + Name and path of the file that is about to be saved
//!     + A progress bar, having the following:
//!         - A amount of KiB or MiB (depending on the download size) that was downloaded
//!         - A percentage of how much was downloaded
//!         - Time that remains to finish the download
//!     + Time that the download finished respecting the previous format
//!
//! It should look something like this
//! ```
//! $ cargo r https://pbs.twimg.com/media/EMtmPFLWkAA8CIS.jpg
//!
//! start at 2017-10-14 03:46:06
//! sending request, awaiting response... status 200 OK
//! content size: 56370 [~0.06MB]
//! saving file to: ./EMtmPFLWkAA8CIS.jpg
//!  55.05 KiB / 55.05 KiB [================================================================================================================] 100.00% 1.24 MiB/s 0s
//!
//! Downloaded [https://pbs.twimg.com/media/EMtmPFLWkAA8CIS.jpg]
//! finished at 2017-10-14 03:46:07
//!
//! $
//! ```

mod flags;

// pub use flags::*;
