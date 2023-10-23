#![allow(unused_variables, dead_code)]

#[derive(Clone, Debug)]
struct FloppyDisk {
    data: String,
}

struct DiskDrive {
    disk: Option<FloppyDisk>,
}

impl DiskDrive {
    fn insert(&mut self, disk: FloppyDisk) {
        if self.disk.is_some() {
            panic!("Disk drive is already occupied");
        }
        self.disk = Some(disk);
    }
    fn eject(&mut self) -> Option<FloppyDisk> {
        self.disk.take()
    }
    fn overwrite(&mut self, data: &str) {
        if let Some(disk) = self.disk.as_mut() {
            disk.data = data.to_string();
        } else {
            panic!("Disk drive is empty");
        }
    }
}

impl FloppyDisk {
    fn new(data: &str) -> Self {
        Self {
            data: data.to_string(),
        }
    }
}

fn main() {}
