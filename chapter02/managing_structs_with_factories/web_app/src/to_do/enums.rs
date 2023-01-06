

pub enum TaskStatus {
    DONE,
    PENDING
}


impl TaskStatus {

    pub fn stringify(&self) -> String {
        match &self {
            &Self::DONE => {"DONE".to_string()},
            &Self::PENDING => {"PENDING".to_string()}
        }
    }
}
