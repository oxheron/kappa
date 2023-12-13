// A bunch of utility functions for the whole compiler

pub use regex::Regex;

// Replaces all newlines & tabs with spaces and makes sure there is only 1 space between things 
pub fn cleanup(data: &mut String) { 
    data.replace("\n", " ");
    data.replace("\t", " ");
    
    let regex = Regex::new(r"\s+").unwrap();
    let data = regex.replace_all(data, " ");   
} 
