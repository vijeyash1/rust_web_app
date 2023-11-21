// TaskStatus is to define the status ,
// we can also define the task status using strings but
// using enums is the idomatic way
pub enum TaskStatus {
    DONE,
    PENDING,
}

// this implementation is for making the enum into a proper
// string to be inserted into a database

impl TaskStatus {
    pub fn stringify(&self) -> String {
        match &self {
            &Self::DONE => "DONE".to_string(),
            &Self::PENDING => "PENDING".to_string(),
        }
    }
    pub fn from_str(input_string: String) -> Self {
        match input_string.as_str() {
            "DONE" => TaskStatus::DONE,
            "PENDING" => TaskStatus::PENDING,
            _ => panic!("input {} is not supported", input_string),
        }
    }
}
