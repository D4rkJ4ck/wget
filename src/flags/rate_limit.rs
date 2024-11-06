//! The program should handle speed limit.
//! Basically the program can control the speed of the download by using the flag `--rate-limit`.
//! If you download a huge file you can limit the speed of your download, preventing the program from using the full possible bandwidth of your connection, example:
//! ```
//! $ cargo r --rate-limit=400k https://pbs.twimg.com/media/EMtmPFLWkAA8CIS.jpg
//! ```
//! This flag should accept different value types, example: k and M. So you can put the rate limit as `rate-limit=200k` or `rate-limit=2M`
