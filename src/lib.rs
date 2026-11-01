pub fn search<'a>(query: &str, content: &'a str)-> Vec<&'a str>{
    let mut result =  Vec::new();
    for line in content.lines(){
        if line.contains(query){
            result.push(line);
        }
    }
    result
}

#[cfg(test)]
mod tests{
  use super::*;
  #[test]
  fn one_result(){
    let query = "duct";
     let content = "\
Rust:
safe, fast, productive.
Pick three.";
assert_eq!(vec!["safe, fast, productive."], search(query, content));
  }
}