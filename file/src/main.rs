//! Rust in Action File implementation
use std::fmt;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

/// Represents a "File" living in a file system
#[derive(Debug)]
pub struct File {
    pub name: String,
    data: Vec<u8>,
    pub state: FileState,
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} {}>", self.name, self.state)
    }
}

impl File {
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    pub fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut f = File::new(name);
        f.data = data.clone();
        f
    }

    pub fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }

        let mut tmp = self.data.clone();
        let read_length = tmp.len();

        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

pub fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}

pub fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    Ok(f)
}

fn main() {
    let mut f1 = File::new_with_data("f1.txt", &vec![114, 117, 115, 116, 33]);
    let mut buffer: Vec<u8> = vec![];

    if f1.read(&mut &mut buffer).is_err() {
        println!("Error checking is working");
    }

    f1 = open(f1).unwrap();
    let f1_length = f1.read(&mut buffer).unwrap();
    f1 = close(f1).unwrap();

    let text = String::from_utf8_lossy(&buffer);
    println!("Debug: {:?}", f1);
    println!("Display: {}", f1);
    println!("{} is {} bytes long", &f1.name, f1_length);
    println!("{}", text);
}
