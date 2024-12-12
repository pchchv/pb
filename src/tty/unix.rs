#[cfg(not(any(target_os = "dragonfly", target_os = "freebsd")))]
fn ioctl_conv<T: Copy>(v: T) -> T {
    v
}
