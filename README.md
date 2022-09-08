# encyclopagia

A library for reading `/proc/kpageflags` and `/proc/[pid]/pagemap` on Linux for
a few different kernel versions, with emphasis on flexibility and
maintainability.

## Features

- [x] Parsing and iterating over `/proc/kpageflags`.
- [x] Explicitly supports multiple Linux kernel versions. Other kernel version
      likely also work, but haven't been tested. The following have been:
	- 3.10
	- 4.15
	- 5.0.8
	- 5.4.0
	- 5.13.0
	- 5.15.0
	- 5.17.0
- [x] Be easily extensible and maintainable to new kernel versions.
- [ ] Read `/proc/[pid]/pagemap`.
