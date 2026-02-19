use serde:: {Deserialize, Serialize};

#[derive(Clone, PartialEq , Serialize, Deserialize , Debug)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Clone , Serialize , Deserialize)]
pub struct Task {
    pub title: String,
    pub status: Status,
}

