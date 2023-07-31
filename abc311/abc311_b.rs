
use proconio::input;


fn main(){

    input! {
        N:usize,
        D:usize,
        S:[String;N],
    }

    let mut count = 0;
    let mut max = 0;
    for i in 0..D{
        let mut flag = true;
        for j in 0..N{

            if S[j].chars().nth(i).unwrap() == 'x'{
                flag = false;
            }
            
        }
        if flag{
            count+=1;
            if count > max{
                max = count;
            }
        }else{
            count = 0;
        }
    }    
    print!("{}",max);

}