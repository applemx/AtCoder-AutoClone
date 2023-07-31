use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;

fn main(){
    input!{
        S:String
    }
    let mut max = 0;
    let mut num = 0;
    for i in S.as_str().chars(){
        if ["A", "C", "G", "T"].contains(&&((i).to_string()[..])) {
            num+=1;
        }else{
            num = 0;
        }
        if(max<num){
            max = num;
        }
    }
    println!("{}",max);
}