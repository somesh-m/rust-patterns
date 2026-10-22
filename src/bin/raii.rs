/*
 * RAII -> Resource Acquisition Is Intialization is a programming idiom that binds the lifecycle of a resource to the lifetime of the object acquiring it. It ensures that critical resources such as memory, filehandles or anything which requires strict acquisition and dropping are automatically acquired when an object is initialized and then released when that object goes out of scope. The release doesn't need the acquirer to specifically call any free or drop functions.
 */

use std::{
    fs::File,
    io::{self, Write},
    thread,
    time::Duration,
};

struct FileGuard {
    file: File,
    owner: String,
}

impl FileGuard {
    fn new(path: &str, owner: String) -> io::Result<Self> {
        let file = File::create(path)?;
        Ok(Self { file, owner })
    }

    fn write(&mut self, data: &[u8]) -> io::Result<()> {
        self.file.write_all(data)
    }
}

impl Drop for FileGuard {
    fn drop(&mut self) {
        println!("Releasing file resource for: {}", self.owner);
    }
}

fn main() -> io::Result<()> {
    let mut guard_b = FileGuard::new("data_b.txt", String::from("Process B"))?;
    {
        let mut guard_a = FileGuard::new("data_a.txt", String::from("Process A"))?;
        guard_a.write(b"Hello from Process A")?;
    }

    println!("Sleeping to show that guard_a exits");
    thread::sleep(Duration::from_secs(2));

    Ok(())
}
