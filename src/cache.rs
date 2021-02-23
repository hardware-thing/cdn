use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub type Cache = Arc<RwLock<HashMap<String, String>>>;
