```rust
fn is_letter(c:char) ->bool{
  if c>='A'&&c<='z'{
    true
  }
  else
  {
    false
  }
}
fn is_digit(c:char) ->bool{
  if c>='0'&&c<='9'{
    true
  }
  else{
    false
  }
}
fn is__(c:char) ->bool{
   if c=='_'{
     true
   }
   else
   {
     false
   }
}
fn lexer_analysis(lex:String)->u32{
  let keywords = vec!["int","return"];
  for keyword in keywords{
    if lex == String::from(keyword){
      return 8;           //represent keyword
    }
  }
  let mut state:u32 = 0;
    for c in lex.chars(){
    if state ==0{
    match c{
     '('=> return 3,
     ')'=> return 4,
     '{'=> return 5,
     '}'=> return 6,
     ';'=> return 7,
     _=> state =0,
    }
    if is_letter(c)||c=='_'{
      state =1;
    }
    else if is_digit(c){
      state =2;
    }
    else{
      return 99999;     //input bug
    }
  }
    else if state == 1{
      if is_digit(c)||is_letter(c)||(c=='_'){
      continue;
    }
      else{
        return 99999;
      }
      
    }
    else if state == 2{
      if is_digit(c){
        continue;
      }
      else{
        return 99999;
      }
    }
  }
  return state;
}


fn main(){
   println!("{}",lexer_analysis(String::from("int")));
   println!("{}",lexer_analysis(String::from("2525")));
   println!("{}",lexer_analysis(String::from("tsinghua")));
}
```

