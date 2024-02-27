use alloc::string::{String, ToString};
use uefi::{proto::media::{file::{Directory, File, FileAttribute, FileHandle, FileMode}, fs::SimpleFileSystem}, table::boot::BootServices, CStr16, Handle};

pub struct FS {
    dir: Directory
}

impl FS {
    pub fn init(bt: &BootServices, _image_handle: Handle) -> Self {
        let fs = bt.get_image_file_system(_image_handle).unwrap();
        let sfs: &mut SimpleFileSystem = fs.get_mut().unwrap();
        let dir: Directory = sfs.open_volume().unwrap();
        FS { dir }
    }
    pub fn open(&mut self, filename: &str) -> (&Self, FileHandle) {
        let mut bf = [0; 4096];
        let f = &CStr16::from_str_with_buf(filename, &mut bf).unwrap();
        let file = self.dir.open(f, FileMode::Read, FileAttribute::ARCHIVE).unwrap();
        (self, file)
    }
    pub fn is_file(file: &FileHandle) -> bool {
        file.is_regular_file().unwrap()
    }
    #[allow(dead_code)]
    pub fn is_dir(file: &FileHandle) -> bool {
        file.is_directory().unwrap()
    }
    pub fn read_string(file: FileHandle) -> String {
        if Self::is_file(&file){
            let rf = &mut file.into_regular_file().unwrap();
            let mut bf: [u8; 4096] = [0; 4096];
            let _ = &rf.read(&mut bf).unwrap();
            let count = rf.get_position().unwrap() as u8;
            let mut str = String::new();
            for i in 0..count as usize{
                let t=char::from_u32(bf[i] as u32).unwrap().to_string();
                str = str + &t;
            }
            str
        }else {
            String::from("")
        }
    }
}