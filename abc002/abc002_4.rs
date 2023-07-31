
use proconio::input;



fn main(){

    input!{
        N:usize,
        M:usize,
        S:[[usize;2];M],
    }


    let mut SS = [[0;12];12];
    for i in 0..M{
            SS[S[i][0]-1][S[i][1]-1] = 1;
    }

    let mut ans = 0;

    for bit in 0..(1<<N){//2048通り(M=12の場合)
        let mut flag = true;
        let mut count = 0;


        

        for i in 0..N{
            if bit & (1<<i) > 0{
                count+=1;
            }
            for j in i..N{
                if bit & (1<<i) > 0 && bit & (1<<j) >0 && i!=j{
                    if SS[i][j] != 1{
                        flag =  false;
                    }
                }
            }
        }
        if flag{
            if ans < count{
                ans = count;
            }
        }
    }

    print!("{}\n",ans);

    
    
}