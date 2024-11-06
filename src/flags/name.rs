//! Download a file and save it under a different name by using the flag `-O` followed by the name you wish to save the file, example:
//! ```
//! $ cargo r -O=meme.jpg https://pbs.twimg.com/media/EMtmPFLWkAA8CIS.jpg
//! 
//! start at 2017-10-14 03:46:06
//! sending request, awaiting response... status 200 OK
//! content size: 56370 [~0.06MB]
//! saving file to: ./meme.jpg
//!  55.05 KiB / 55.05 KiB [================================================================================================================] 100.00% 1.24 MiB/s 0s
//! 
//! Downloaded [https://pbs.twimg.com/media/EMtmPFLWkAA8CIS.jpg]
//! finished at 2017-10-14 03:46:07
//! 
//! $ ls -l
//! 
//! -rw-r--r-- 1 student student 56370 ago 13 16:59 meme.jpg
//! -rw-r--r-- 1 student student 11489 ago 13 10:28 main.go
//! 
//! $
//! ```

