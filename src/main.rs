use std::io::{self, Read, Write};
use std::env;
use pinyin::{ ToPinyin };

fn chars_to_pinyin(chars: String) -> String {
    let mut pinyins: Vec<String> = Vec::new();

    let mut need_delemiter = false;
    for char in chars.split("") {
      for pinyin in char.to_pinyin() {
          if let Some(pinyin) = pinyin {
              if need_delemiter {
                pinyins.push(String::from("-"));
              }
              pinyins.push(pinyin.plain().to_string());
              pinyins.push(String::from("-"));
              need_delemiter = false;
          } else {
              pinyins.push(String::from(char));
              need_delemiter = true;
          }
      }
    }

    // remove last 
    let last_char = pinyins.pop().unwrap_or_default();
    if last_char != String::from("-") {
      pinyins.push(last_char);
    }

    pinyins.join("")
}

fn read_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer.trim().to_string())
}

fn main() -> io::Result<()> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let mut chars = if args.len() == 0 {
        read_stdin()?
    } else {
        args.join(" ")
    };

    let out = chars_to_pinyin(chars) + "\n";
    let result = out.as_bytes();
    io::stdout().write_all(result)?;

    Ok(())
}
