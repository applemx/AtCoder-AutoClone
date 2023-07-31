
use proconio::input;

fn main(){
    input!{
        n:usize,
        p:usize,
        q:usize,
        d:[usize;n],
    }
    let mut min = 1000000;

    for i in 0..n{
        if d[i]<min{
            min = d[i];
        }
        
    }
    min += q;
    if min>p{
        min = p;
    }
    print!("{}",min);
}