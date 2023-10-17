#[derive(Default, Debug, Copy, Clone)]
pub struct InstallProgress {
    pub current: usize,
    pub total: usize,
}

impl InstallProgress {
    pub const NOT_STARTED: Self = Self {
        current: 0,
        total: 0,
    };
}
