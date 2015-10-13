# make-cmd-rs
Intended as a build-dependency of cargo packages that want to use "make" from build.rs scripts on different platforms.

This fixes build  issues. For example "make" in FreeBSD refers to the BSD make,
while "make" on Linux is GNU make.
