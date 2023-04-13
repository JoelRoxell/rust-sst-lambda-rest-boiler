pub struct InternalService {
    title: String,
}

impl InternalService {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
        }
    }

    pub fn set(&mut self, title: &str) {
        self.title = title.to_string();
    }

    pub fn get(&self) -> String {
        self.title.clone()
    }
}
