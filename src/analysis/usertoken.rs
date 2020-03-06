use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use chrono::prelude::*;
use lazy_static::lazy_static;
use crate::analysis::apisum::TokenApiSum;

//初始化正则表达式
lazy_static! {
    static ref RE: Regex = Regex::new(r"Bearer\s\S+").unwrap();
}

//统计每个token对应访问API的数量
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
    println!("共多少条记录:{}",k);
    get_top_ten_token(user_map);
}

//获取今天日志文件,只处理今天日志文件
pub fn get_today_gateway_files()->Vec<String>{
    let dir = "/home/blackvip/logs/gateway/";
    let mut files:Vec<String> = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let filename = entry.file_name();
                let filename:String = filename.into_string().unwrap();
                let filename = filename.as_str();
                let today_str = get_today_date_str();
                let today_file_prefix = "count_user.".to_owned()+&today_str;
                let is_today_file = filename.starts_with(&today_file_prefix);
                let current_file = filename.starts_with("count_user_token_file.log");
                if is_today_file||current_file{
                    println!(" 需要分析的日志文件====>filename:{}",filename);
                    let fullpathfile = dir.to_owned()+filename;
                    files.push(fullpathfile);
                }
            }
        }
    }
    files
}

//获取今天日期,返回格式:2020-02-16
fn get_today_date_str()-> String{
    let local: DateTime<Local> = Local::now();
    let datastr = local.date().to_string();
    let datastr = &datastr[0..10];
    String::from(datastr)
}


//获取排行访问量前10的token
fn get_top_ten_token(mut tokens:HashMap<String,u32>){
    let mut ten_token_people:Vec<TokenApiSum> = Vec::new();
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
        let token = String::from(&remove_key);
        let max_apisum_struct = TokenApiSum::new(token,max_value);
        ten_token_people.push(max_apisum_struct);
        tokens.remove(remove_key.as_str());
    }
    for item in ten_token_people {
        println!("top ten people:{:?}",item);
    }
}

//统计每个token的总访问量
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

//统计每个文件里每个token的访问量
fn single_file_sum_token(file:String)->Vec<String>{
    let mut tokens:Vec<String> = Vec::new();
    let file = File::open(file).unwrap();
    let f = BufReader::new(file);
    let mut  m = 0;
    for line in f.lines() {
        let line_str = line.unwrap();
        let line_tokens = reg_line(line_str);
        for i in line_tokens {
            tokens.push(i);
        }
        m = m +1;
    }
    tokens
}

//按行分析是否匹配正则表达式的模式
pub fn reg_line(line:String)->Vec<String>{
    let mut line_tokens:Vec<String> = Vec::new();
    let mut i = 0;
    for caps in RE.captures_iter(line.as_str()) {
        let token = caps.get(0).unwrap().as_str();
        line_tokens.push(token.to_owned());
        i  = i+1;
    }
    line_tokens
    
}