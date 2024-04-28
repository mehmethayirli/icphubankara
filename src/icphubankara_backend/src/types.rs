use candid::{CandidType, Deserialize};

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct TodoStorage{
        pub todos: Vec<Todo>,
}

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct Todo{
    pub description: String,
    pub done: bool,
}