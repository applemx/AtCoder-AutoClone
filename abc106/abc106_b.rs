use itertools::Itertools;
use proconio::{input, marker::*};
use std::cmp::*;

fn main(){
    input!{
        n:i64
    };
    let mut sum=0;
    for i in 1..=n{
        let mut count =0;
        if i%2==0 {
            continue;
        }
        for j in 1..=n{
            if i%j==0{
                count+=1;
            }
        }
        if(count == 8){
            sum +=1;
        }
    }
    

    println!("{}",sum);
}