
use proconio::input;


fn main(){

    input! {
        N:usize,
        S:String,
    }
    
    let mut flaga = false;
    let mut flagb = false;
    let mut flagc = false;
    for i in 0..N{
        if S.chars().nth(i).unwrap() == 'A'{
            flaga = true;
        }

        if S.chars().nth(i).unwrap()== 'B'{
            flagb = true;
        }

        if S.chars().nth(i).unwrap() == 'C'{
            flagc = true;
        }

        if flaga && flagb && flagc{
            print!("{}",i+1);
            break;
        }
    }
}