use regex::Regex;
use std::{collections::HashMap, error::Error, fs};

pub fn load(path: &str) -> Result<Info, Box<dyn Error>> {
    parse(&fs::read_to_string(path)?)
}

pub fn parse(data: &str) -> Result<Info, Box<dyn Error>> {
    let line_pattern = Regex::new(r"(?m)^([a-z]{2,8})\s+(.*)$")?;

    let mut label_to_addr: HashMap<String, u16> = HashMap::new();
    let mut addr_to_label: HashMap<u16, String> = HashMap::new();

    for (_, [keyword, pairstr]) in line_pattern.captures_iter(&data).map(|c| c.extract()) {
        match keyword {
            "sym" => {
                let mut label: Option<String> = None;
                let mut addr: Option<u16> = None;
                for (k, v) in pairstr.split(",").map(|p| p.split_once("=").unwrap()) {
                    match k {
                        "name" => label = Some(v.trim_matches('"').to_string()),
                        "val" => {
                            addr = Some(u16::from_str_radix(v.strip_prefix("0x").unwrap(), 16)?)
                        }
                        _ => {}
                    }
                }
                if let (Some(label), Some(addr)) = (label, addr) {
                    label_to_addr.insert(label.clone(), addr);
                    addr_to_label.insert(addr, label.clone());
                }
            }
            _ => {}
        }
    }

    Ok(Info {
        label_to_addr,
        addr_to_label,
    })
}

#[allow(unused)]
pub struct Info {
    pub label_to_addr: HashMap<String, u16>,
    pub addr_to_label: HashMap<u16, String>,
}

#[allow(unused)]
impl Info {
    pub fn addr(&self, addr: &str) -> Option<u16> {
        self.label_to_addr.get(addr).copied()
    }

    pub fn label(&self, addr: u16) -> Option<&str> {
        self.addr_to_label.get(&addr).map(|x| x.as_str())
    }
}
