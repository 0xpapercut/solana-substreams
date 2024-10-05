#![allow(deprecated)]

use regex;
use base64;

#[derive(Debug)]
pub enum Log<'a> {
    Invoke(InvokeLog<'a>), // "Program {} invoke [{}]",
    Success(SuccessLog<'a>), // Program {} success
    Return(ReturnLog<'a>), // "Program return: {} {}"
    Data(DataLog<'a>), //  "Program data: {}"
    Program(ProgramLog<'a>), // "Program log: {}"
    Unknown(UnknownLog<'a>),
}

impl<'a> Log<'a> {
    pub fn new(log: &'a String) -> Self {
        if log.starts_with("Program log: ") {
            return Self::Program(ProgramLog::new(log));
        }
        if log.starts_with("Program data: ") {
            return Self::Data(DataLog::new(log));
        }
        if log.starts_with("Program return: ") {
            return Self::Return(ReturnLog::new(log));
        }
        if log.split_whitespace().count() == 4 && log.starts_with("Program") && log.split_whitespace().nth(2) == Some("invoke") {
            return Self::Invoke(InvokeLog::new(log));
        }
        if log.split_whitespace().count() == 3 && log.starts_with("Program") && log.ends_with("success") {
            return Self::Success(SuccessLog::new(log))
        }
        Self::Unknown(UnknownLog::new(log))
    }

    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success(_))
    }
    pub fn is_invoke(&self) -> bool {
        matches!(self, Self::Invoke(_))
    }

    pub fn is_return(&self) -> bool {
        matches!(self, Self::Return(_))
    }

    pub fn is_data(&self) -> bool {
        matches!(self, Self::Data(_))
    }

    pub fn is_program(&self) -> bool {
        matches!(self, Self::Program(_))
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown(_))
    }
}

#[derive(Debug)]
pub struct ProgramLog<'a> {
    log: &'a String,
}

impl<'a> ProgramLog<'a> {
    fn new(log: &'a String) -> Self {
        Self { log }
    }
    pub fn message(&self) -> Result<String, String> {
        let re = regex::Regex::new(r"Program log: (.+)").unwrap();
        if let Some(captures) = re.captures(self.log) {
            let message = captures.get(1).unwrap().as_str().to_string();
            Ok(message)
        } else {
            Err("Error parsing ProgramLog.".into())
        }
    }
}

#[derive(Debug)]
pub struct InvokeLog<'a> {
    log: &'a String,
}

impl<'a> InvokeLog<'a> {
    pub fn new(log: &'a String) -> Self {
        Self { log }
    }
    pub fn program_id(&self) -> Result<String, String> {
        let re = regex::Regex::new(r"Program (.+) invoke \[(\d+)\]").unwrap();
        if let Some(captures) = re.captures(self.log) {
            let program_id = captures.get(1).unwrap().as_str().to_string();
            Ok(program_id)
        } else {
            Err("Error parsing InvokeLog.".into())
        }
    }

    pub fn invoke_depth(&self) -> Result<u32, String> {
        let re = regex::Regex::new(r"Program (.+) invoke \[(\d+)\]").unwrap();
        if let Some(captures) = re.captures(self.log) {
            let invoke_depth = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
            Ok(invoke_depth)
        } else {
            Err("Error parsing InvokeLog.".into())
        }
    }
}

#[derive(Debug)]
pub struct SuccessLog<'a> {
    pub log: &'a String,
}

impl<'a> SuccessLog<'a> {
    pub fn new(log: &'a String) -> Self {
        Self { log }
    }
    pub fn program_id(&self) -> Result<String, String> {
        let re = regex::Regex::new(r"Program (.+) success").unwrap();
        if let Some(captures) = re.captures(self.log) {
            let program_id = captures.get(1).unwrap().as_str().to_string();
            Ok(program_id)
        } else {
            Err("Error parsing SuccessLog.".into())
        }
    }
}

#[derive(Debug)]
pub struct ReturnLog<'a> {
    pub log: &'a String,
}

impl<'a> ReturnLog<'a> {
    pub fn new(log: &'a String) -> Self {
        Self { log }
    }
    pub fn program_id(&self) -> Result<String, String> {
        let re = regex::Regex::new(r"Program return: (.+) (.+)").unwrap();
        if let Some(captures) = re.captures(self.log) {
            let program_id = captures.get(1).unwrap().as_str().to_string();
            Ok(program_id)
        } else {
            Err("Error parsing ReturnLog.".into())
        }
    }

    pub fn data(&self) -> Result<Vec<u8>, String> {
        let re = regex::Regex::new(r"Program return: (.+) (.+)").unwrap();
        if let Some(captures) = re.captures(self.log) {
            let encoded_data = captures.get(2).unwrap().as_str();
            base64::decode(encoded_data).map_err(|_| String::from("Base64 decoding error."))
        } else {
            Err("Error parsing ReturnLog.".into())
        }
    }
}

#[derive(Debug)]
pub struct DataLog<'a> {
    pub log: &'a String,
}

impl<'a> DataLog<'a> {
    pub fn new(log: &'a String) -> Self {
        Self { log }
    }
    pub fn data(&self) -> Result<Vec<u8>, String> {
        let re = regex::Regex::new(r"Program data: (.+)").unwrap();
        if let Some(captures) = re.captures(self.log) {
            let encoded_data = captures.get(1).unwrap().as_str();
            base64::decode(encoded_data).map_err(|_| String::from("Base64 decoding error."))
        } else {
            Err("Error parsing DataLog.".into())
        }
    }
}

#[derive(Debug)]
pub struct UnknownLog<'a> {
    pub log: &'a String,
}

impl<'a> UnknownLog<'a> {
    pub fn new(log: &'a String) -> Self {
        Self { log }
    }
}
