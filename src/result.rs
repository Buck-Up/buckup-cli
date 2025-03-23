use std::error::Error;

pub type SmartSyncResult = Result<(), Box<dyn Error>>;
