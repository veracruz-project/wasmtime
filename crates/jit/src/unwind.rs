cfg_if::cfg_if! {
    if #[cfg(all(windows, target_arch = "x86_64"))] {
        mod winx64;
        pub use self::winx64::*;
    } else if #[cfg(all(windows, target_arch = "x86"))] {
        mod winx32;
        pub use self::winx32::*;
    } else if #[cfg(unix)] {
        mod systemv;
        pub use self::systemv::*;
    } else if #[cfg(target_os = "icecap")] {
        mod icecap;
        pub use self::icecap::*;
    } else {
        compile_error!("unsupported target platform for unwind");
    }
}
