use libc;

extern "C" {
    fn clear_cache(start: *const u8, end: *const usize);
}

pub unsafe fn flush(start_addr: *const u8, len: usize) -> std::io::Result<()> {
    unsafe { *libc::__errno_location() = 0 };
    let end = start_addr.wrapping_add(len);
    clear_cache(start_addr, end as *const usize);
    let e = std::io::Error::last_os_error();
    match e.raw_os_error() {
        Some(errno) if errno != 0 => Err(e),
        Some(_) => Ok(()),
        None => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(arch="x86_64")]
    fn it_works_always_on_x86_64() {
        unsafe {
            flush(0xffff00 as *const u8, 4096).unwrap();
        }
    }

    #[test]
    #[cfg(not(arch="x86_64"))]
    fn it_works_on_non_x86_with_good_values() {
        unsafe {
            flush(it_works_on_non_x86_with_good_values as *const u8, 4096).unwrap();
        }
    }
}
