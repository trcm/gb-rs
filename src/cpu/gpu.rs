#[allow(dead_code)]
pub struct GPU {
    status: bool
}

#[allow(dead_code)]
impl GPU {
    pub fn new() -> GPU {
        let gpu = GPU {
            status: false
        };
        gpu
    }

    pub fn status(&self) -> bool {
        self.status
    }
}
