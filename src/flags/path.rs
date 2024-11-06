//! It should also handle the path to where your file is going to be saved using the flag `-P` followed by the path to where you want to save the file, example:
//! ```
//! $ cargo r -P=~/Downloads/ -O=meme.jpg https://pbs.twimg.com/media/EMtmPFLWkAA8CIS.jpg
//!
//! start at 2017-10-14 03:46:06
//! sending request, awaiting response... status 200 OK
//! content size: 56370 [~0.06MB]
//! saving file to: ~/Downloads/meme.jpg
//!  55.05 KiB / 55.05 KiB [================================================================================================================] 100.00% 1.24 MiB/s 0s
//!
//! Downloaded [https://pbs.twimg.com/media/EMtmPFLWkAA8CIS.jpg]
//! finished at 2017-10-14 03:46:07
//!
//! $ ls -l ~/Downloads/meme.jpg
//!
//! -rw-r--r-- 1 student student 56370 ago 13 16:59 /home/student/Downloads/meme.jpg
//!
//! $
//! ```
