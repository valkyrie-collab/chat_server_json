use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Client {
    thread_id: usize,
    username: Option<String>,
    message: Option<String>
}

impl Client {
    pub fn new() -> Self {
        Client {
            thread_id: 0,
            username: None,
            message: None 
        }
    }
    
    pub fn ref_server_id(&self) -> &usize {
        &self.thread_id
    }
    
    pub fn mut_username(&mut self) -> Option<&mut String> {
        
        if let Some(username) = &mut self.username {
            return Some(username);
        }
        
        None
    }
    
    pub fn ref_username(&self) -> Option<&String> {
        
        if let Some(username) = &self.username {
            return Some(username);
        }
        
        None
    }
    
    pub fn mut_message(&mut self) -> Option<&mut String> {
        
        if let Some(message) = &mut self.message {
            return Some(message);
        }
        
        None
    }
    
    pub fn ref_message(&self) -> Option<&String> {
        
        if let Some(message) = &self.message {
            return Some(message);
        }
        
        None
    }
    
    pub fn set_id(&mut self, id: usize) {
        self.thread_id = id;
    }
    
    pub fn set_username(&mut self, username: String) {
        self.username = Some(username);
    }
    
    pub fn set_message(&mut self, message: String) {
        self.message = Some(message);
    }
}