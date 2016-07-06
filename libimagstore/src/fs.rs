//! A wrapper module around the std::fs API so we can replace the functions with own
//! implementations to do testing in a neat way.
//!
//! The normal implementations (for non-test builds) simply call the std::fs equivalents which
//! should be inlined by the compiler then, so we have zero costs for this thin private layer over
//! std::fs.

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

