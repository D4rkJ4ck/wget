//! The flag `-B` should be handled, this flag should download a file immediately to the background and the output should be redirected to a log file.
//! When the program containing this flag is executed it should output : `Output will be written to "wget-log"`.
//! Example:
//! ```
//! $ cargo r -B https://pbs.twimg.com/media/EMtmPFLWkAA8CIS.jpg
//!
//! Output will be written to "wget-log".
//!
//! $ cat wget-log
//!
//! start at 2017-10-14 03:46:06
//! sending request, awaiting response... status 200 OK
//! content size: 56370 [~0.06MB]
//! saving file to: ./EMtmPFLWkAA8CIS.jpg
//! Downloaded [https://pbs.twimg.com/media/EMtmPFLWkAA8CIS.jpg]
//! finished at 2017-10-14 03:46:07
//!
//! $
//! ```
