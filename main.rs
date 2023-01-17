// P: https://www.acmicpc.net/problem/16496
use std::io::stdin;
fn main(){
    let mut s = String::new();
    stdin().read_line(&mut s).unwrap();

    s = String::new();
    stdin().read_line(&mut s).unwrap();
    let mut res = s.split_whitespace().collect::<Vec<&str>>();
    res.sort_by_key(|x|x.len());
    let len = res[res.len()-1].len();
    let repeat = (len as f32 / res[0].len() as f32).round() as usize + 1;

    res.sort_by_key(|x|{(x.repeat(repeat*2)[..len*2]).parse::<u128>().unwrap()});
    res.reverse();
    let mut end = String::new();
    for s in res{
        end.push_str(s);
    }
    if end.replace("0", "") == ""{
        print!("0")
    }else{
        print!("{end}")
    }
}
