use std::fs::OpenOptions;
use std::io::*;
use std::mem;
use std::ops::Rem;
use std::ptr;

pub fn rand<T: Copy>() -> T {
    let mut gen = OpenOptions::new().read(true).open("/dev/urandom").unwrap();
    let mut buf = vec![0_u8; mem::size_of::<T>()];
    gen.read_exact(&mut buf).unwrap();
    unsafe {
        let mut value = mem::MaybeUninit::uninit();
        ptr::copy(
            buf.as_ptr(),
            ptr::addr_of_mut!(value) as _,
            mem::size_of::<T>(),
        );
        value.assume_init()
    }
}

pub fn rand_int<T: Copy + Rem<T, Output = T>>(to: T) -> T {
    rand::<T>() % to
}

fn k_permutation<T>(seq: &mut [T]) {
    seq[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rand() {
        let _: usize = rand();
    }
}
