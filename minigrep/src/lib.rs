use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
}

// 使用面向对象的方式编程，让数据结构更清晰
impl Config {
    // 使用 Result 的方式来处理
    pub fn build(args: &[String]) -> Result<Config,&'static str>{
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config{ query, file_path })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn on_result() {
        let query = "duct";
        let contents = "\
         Rust:
safe, fast, productive.
         Pick three.
        ";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

/*
1. 遍历迭代 contents 的每一行
2. 检查该行内容是否包含我们的目标字符串
3. 若包含，则放入返回值列表中，否则忽略
4. 返回匹配到的返回值列表
*/
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}