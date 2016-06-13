/// # Example
///
///```
/// println!("pointer width {}", pointer_width::POINTER_WIDTH)
///```

#[cfg(target_pointer_width = "16")]
pub const POINTER_WIDTH: usize = 16;
#[cfg(target_pointer_width = "32")]
pub const POINTER_WIDTH: usize = 32;
#[cfg(target_pointer_width = "64")]
pub const POINTER_WIDTH: usize = 64;
#[cfg(target_pointer_width = "128")]
pub const POINTER_WIDTH: usize = 128;
