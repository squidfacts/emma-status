use std::thread;
use std::time::Duration;
use std::io::{self,Write};
use std::fs;
use chrono::Local;
use regex::Regex;
use dotenvy::dotenv;

fn formatted_now() -> String {
    Local::now().format("%Y.%m.%d %-I:%M%P").to_string()
}

fn get_daily_todo_path(path: String) -> String {
    let today = Local::now().format("%Y-%m-%d");
    format!("{}{}.md",path, today)
}

fn count_todos(path: String) -> (usize, usize) {
    let file_path = get_daily_todo_path(path);
    
    match fs::read_to_string(&file_path) {
        Ok(content) => {
            let done_regex = Regex::new(r"- \[x\]").unwrap();
            let undone_regex = Regex::new(r"- \[ \]").unwrap();
            
            let done_count = done_regex.find_iter(&content).count();
            let undone_count = undone_regex.find_iter(&content).count();
            
            (done_count, undone_count)
        },
        Err(_) => (0, 0)
    }
}

fn print_stuff(path: String) -> String {
    let time = formatted_now();
    let (done_count, undone_count) = count_todos(path);
    
    let mut parts = vec![
        format!("{{\"full_text\":\"{time}\", \"name\":\"time\",\"color\":\"#BF40BF\"}}"),
        format!("{{\"full_text\":\"Done: {done_count}\", \"name\":\"done\",\"color\":\"#00FF00\"}}"),
    ];
    
    if undone_count > 0 {
        parts.push(format!("{{\"full_text\":\"Undone: {undone_count}\", \"name\":\"undone\",\"color\":\"#FF0000\"}}"));
    }
    
    format!("[{}]", parts.join(","))
}

fn main() {
    dotenv().unwrap();

    let mut daily_path: String = dotenvy::var("DAILY_VAULT_PATH").unwrap();

    if daily_path.chars().last().unwrap() != '/' {
        daily_path = daily_path + "/";
    }
    let second = Duration::from_secs(1);
    println!("{{\"version\":1, \"click_events\": true}}");
    println!("[");
    io::stdout().flush().unwrap();

    println!("{}",print_stuff(daily_path.clone()));
    io::stdout().flush().unwrap();
    loop {
        println!(",{}", print_stuff(daily_path.clone()));
        io::stdout().flush().unwrap();
        thread::sleep(second);
    }
}
