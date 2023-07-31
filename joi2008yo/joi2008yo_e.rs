
use proconio::input;



fn main(){

    input!{
        R:usize,
        C:usize,
        S:[[usize;C];R],
    }
    let mut ans = 0;
    for bit in 0..(1<<R){
        let mut SS = S.clone();
        for i in 0..R{
            if bit & (1<<i) > 0{
                for j in 0..C{
                    if SS[i][j] == 1{
                        SS[i][j] = 0;
                    }else{
                        SS[i][j] = 1;
                    }
                }
            }
        }
        
        for k in 0..C{
            let mut flagcount = 0;
            for l in 0..R{
                if SS[l][k] == 1{
                    flagcount-=1;
                }else{
                    flagcount+=1;
                }
            }
            if flagcount <0{
                for l in 0..R{
                    if SS[l][k] == 1{
                        SS[l][k] = 0;
                    }else{
                        SS[l][k] = 1;
                    }
                }
            }
        }

        let mut count = 0;
        for k in 0..C{
            for l in 0..R{
                if SS[l][k] == 0{
                    count+=1;
                }
            }
        }

        if count > ans{
            ans = count;
        }

    }
    print!("{}\n",ans);
}