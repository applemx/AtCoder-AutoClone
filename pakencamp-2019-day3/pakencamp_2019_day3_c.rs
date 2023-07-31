use itertools::Itertools;
use proconio::input;

 
fn main(){
    input!{
        n: usize,
        m: usize,
        a: [[i64;m ];n],
    }
    
    let mut max=0;
    for i in 0..m-1{
        for j in i+1..m{
            let mut num = 0;
            for k in 0..n{
                if (a[k][i]>a[k][j]){
                    num+=a[k][i];
                }else{
                    num+=a[k][j];
                }
            }
            if(num>max){
                max=num;
            }
        }   
    }
    println!("{}",max);
}