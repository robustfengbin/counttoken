use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use std::collections::HashMap;

use chrono::prelude::*;

use lazy_static::lazy_static;


lazy_static! {
    static ref RE: Regex = Regex::new(r"Bearer\s\S+").unwrap();
}

pub fn statistic_tokens(){
    let files = get_today_gateway_files();
    let tokens_vec = sum_token(files);
    let  mut k = 0;
    let mut user_map:HashMap<String,u32> = HashMap::new();
    for t in tokens_vec {
        k = k+1;
        if user_map.contains_key(&t){
            let size =  user_map.get(&t).unwrap();
            let new = *size +1;
            user_map.insert(String::from(t), new);
            
        }else {
            
            user_map.insert(t, 1);
        }
    }
    println!("k:{}",k);
    get_top_ten_token(user_map);
}
pub fn get_today_gateway_files()->Vec<String>{
    let  dir = "/Users/apple/logs/gw_cu/";
    // let dir = "/home/blackvip/logs/gateway/";
    let mut files:Vec<String> = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let filename = entry.file_name();
                let filename:String = filename.into_string().unwrap();
                let filename = filename.as_str();
                println!("{:?}", entry.file_name());
                // let today_str = get_today_date_str();
                let today_str = "2020-03-05".to_owned();
                let today_file_prefix = "count_user.".to_owned()+&today_str;
                let is_today_file = filename.starts_with(&today_file_prefix);
                let current_file = filename.starts_with("count_user_token_file.log");
                if is_today_file||current_file{
                    println!(" need perform file====>filename:{}",filename);
                    let fullpathfile = dir.to_owned()+filename;
                    files.push(fullpathfile);
                }
            }
        }
    }
    files
}

fn get_today_date_str()-> String{
    let local: DateTime<Local> = Local::now();
    let datastr = local.date().to_string();
    let datastr = &datastr[0..10];
    String::from(datastr)
}



fn get_top_ten_token(mut tokens:HashMap<String,u32>){
    
    let mut ten_token_map:HashMap<String,u32> = HashMap::new();
    
    for _ in 0..10 {
        
        let  mut max_value:u32 = 0;
        let mut remove_key = String::new();
        for (ke,val)in &tokens {
            if val> &max_value {
                remove_key = String::from(ke);
                let tem_val = val.to_string();
                let tt = tem_val.parse::<u32>().unwrap();
                max_value = tt;
            }
        }
        ten_token_map.insert(String::from(&remove_key), max_value);
        tokens.remove(remove_key.as_str());
    }
    
    for (key,value) in ten_token_map {
        println!("top ten key:{} value:{}",key,value);
    }
    
    
}

fn sum_token(files:Vec<String>)->Vec<String>{
    let  mut   tokens_vec:Vec<String> = Vec::new();
    for file in files {
        let single_file_token_vec =  single_file_sum_token(file);
        for  m in single_file_token_vec {
            tokens_vec.push(m);
        }
    }
    tokens_vec
}

fn single_file_sum_token(file:String)->Vec<String>{
    let mut tokens:Vec<String> = Vec::new();
    let file = File::open(file).unwrap();
    println!("file===================>:{:?}",&file);
    let f = BufReader::new(file);
    
    let mut  m = 0;
    for line in f.lines() {
        let line_str = line.unwrap();
        let line_tokens = reg_line(line_str);
        for i in line_tokens {
            //    println!("1111111----->i:{}",i);
            tokens.push(i);
        }
        // if m%1000== 0{
        //     println!("执行完成1000条记录:{}",m);
        // }
        m = m +1;
    }
    tokens
}

pub fn reg_line(line:String)->Vec<String>{
    // let re = Regex::new(r"Bearer\s\S+").unwrap();
    let mut line_tokens:Vec<String> = Vec::new();
    let mut i = 0;
    for caps in RE.captures_iter(line.as_str()) {
        let token = caps.get(0).unwrap().as_str();
        // println!("token********=>{}",token);
        line_tokens.push(token.to_owned());
        i  = i+1;
        
    }
    line_tokens
    
}