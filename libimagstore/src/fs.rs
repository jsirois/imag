//! A wrapper module around the std::fs API so we can replace the functions with own
//! implementations to do testing in a neat way.
//!
//! The normal implementations (for non-test builds) simply call the std::fs equivalents which
//! should be inlined by the compiler then, so we have zero costs for this thin private layer over
//! std::fs.

#[cfg(test)]
pub mod filesystem {
    use std::sync::{Arc, Mutex};
    use std::io::Result;
    use std::path::Path;
    use std::path::PathBuf;
    use std::fs::File as FSFile;
    use std::collections::HashMap;
    use std::fs::Metadata;
    use std::io::{Seek, SeekFrom, Write, Read};

    use super::file::File;

    use self::default::DefaultFS;

    static filesystem : Arc<Mutex<Box<FileSystem>>> = Arc::new(Mutex::new(Box::new(DefaultFS::new())));

    pub trait FileSystem {
        fn remove_file<P: AsRef<Path>>(&self, path: P)    -> Result<()>;
        fn create_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<()>;
        fn file_open<P: AsRef<Path>>(path: P)             -> Result<FSFile>;
        fn file_create<P: AsRef<Path>>(path: P)           -> Result<FSFile>;
        fn file_sync_all(&self)                           -> Result<()>;
        fn file_sync_data(&self)                          -> Result<()>;
        fn file_set_len(&self, size: u64)                 -> Result<()>;
        fn file_metadata(&self)                           -> Result<Metadata>;
        fn file_try_clone(&self)                          -> Result<FSFile>;
        fn file_read(&mut self, buf: &mut [u8])           -> Result<usize>;
        fn file_write(&mut self, buf: &[u8])              -> Result<usize>;
        fn file_flush(&mut self)                          -> Result<()>;
        fn file_seek(&mut self, pos: SeekFrom)            -> Result<u64>;
    }

    pub mod default {

        pub struct DefaultFS {
            hm: HashMap<PathBuf, String>,
        }

        impl DefaultFS {
            fn new() -> DefaultFS {
                DefaultFS {
                    hm: HashMap::new()
                }
            }
        }

        impl FileSystem for DefaultFS {

            pub fn remove_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
                unimplemented!()
            }

            pub fn create_dir_all<P: AsRef<Path>>(&self, path: P) -> Result<()> {
                unimplemented!()
            }

            pub fn file_open<P: AsRef<Path>>(path: P) -> Result<FSFile> {
                unimplemented!()
            }

            pub fn file_create<P: AsRef<Path>>(path: P) -> Result<FSFile> {
                unimplemented!()
            }

            pub fn file_sync_all(&self) -> Result<()> {
                unimplemented!()
            }

            pub fn file_sync_data(&self) -> Result<()> {
                unimplemented!()
            }

            pub fn file_set_len(&self, size: u64) -> Result<()> {
                unimplemented!()
            }

            pub fn file_metadata(&self) -> Result<Metadata> {
                unimplemented!()
            }

            pub fn file_try_clone(&self) -> Result<FSFile> {
                unimplemented!()
            }

            pub fn file_read(&mut self, buf: &mut [u8]) -> Result<usize> {
                unimplemented!()
            }

            pub fn file_write(&mut self, buf: &[u8]) -> Result<usize> {
                unimplemented!()
            }

            pub fn file_flush(&mut self) -> Result<()> {
                unimplemented!()
            }

            pub fn file_seek(&mut self, pos: SeekFrom) -> Result<u64> {
                unimplemented!()
            }
        }

    }
}

mod remove_file {
    use std::io::Result;
    use std::path::Path;

    #[cfg(test)]
    pub fn remove_file<P: AsRef<Path>>(path: P) -> Result<()> {
        unimplemented!()
    }

    #[cfg(not(test))]
    pub fn remove_file<P: AsRef<Path>>(path: P) -> Result<()> {
        use std::fs::remove_file as std_remove_file;
        std_remove_file(path)
    }
}
pub use self::remove_file::remove_file;

mod create_dir_all {
    use std::io::Result;
    use std::path::Path;

    #[cfg(test)]
    pub fn create_dir_all<P: AsRef<Path>>(path: P) -> Result<()> {
        unimplemented!()
    }

    #[cfg(not(test))]
    pub fn create_dir_all<P: AsRef<Path>>(path: P) -> Result<()> {
        use std::fs::create_dir_all as std_create_dir_all;
        std_create_dir_all(path)
    }
}
pub use self::create_dir_all::create_dir_all;

mod file {
    use std::fs::File as FSFile;

    #[cfg(not(test))]
    use std::ops::{Deref, DerefMut};

    #[cfg(test)]
    use std::io::Result;
    #[cfg(test)]
    use std::io::{Seek, SeekFrom, Write, Read};
    #[cfg(test)]
    use std::fs::Metadata;
    #[cfg(test)]
    use std::path::Path;

    #[derive(Debug)]
    pub struct File(FSFile);

    #[cfg(test)]
    impl File {
        pub fn open<P: AsRef<Path>>(path: P) -> Result<FSFile> {
            unimplemented!()
        }

        pub fn create<P: AsRef<Path>>(path: P) -> Result<FSFile> {
            unimplemented!()
        }

        pub fn sync_all(&self) -> Result<()> {
            unimplemented!()
        }

        pub fn sync_data(&self) -> Result<()> {
            unimplemented!()
        }

        pub fn set_len(&self, size: u64) -> Result<()> {
            unimplemented!()
        }

        pub fn metadata(&self) -> Result<Metadata> {
            unimplemented!()
        }

        pub fn try_clone(&self) -> Result<FSFile> {
            unimplemented!()
        }
    }

    #[cfg(not(test))]
    impl Deref for File {
        type Target = FSFile;

        fn deref(&self) -> &FSFile {
            &self.0
        }

    }

    #[cfg(not(test))]
    impl DerefMut for File {

        fn deref_mut(&mut self) -> &mut FSFile {
            &mut self.0
        }

    }

    #[cfg(test)]
    impl Read for File {

        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            unimplemented!()
        }

    }

    #[cfg(test)]
    impl Write for File {

        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            unimplemented!()
        }

        fn flush(&mut self) -> Result<()> {
            unimplemented!()
        }

    }

    #[cfg(test)]
    impl<'a> Read for &'a File {

        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            unimplemented!()
        }

    }

    #[cfg(test)]
    impl<'a> Write for &'a File {

        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            unimplemented!()
        }

        fn flush(&mut self) -> Result<()> {
            unimplemented!()
        }

    }

    #[cfg(test)]
    impl Seek for File {

        fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
            unimplemented!()
        }

    }

    impl From<FSFile> for File {

        fn from(file: FSFile) -> File {
            File(file)
        }

    }

}
pub use self::file::File;

