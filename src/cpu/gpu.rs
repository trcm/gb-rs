pub struct GPU {
    status: bool
}

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
