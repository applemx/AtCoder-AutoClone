use proconio::input;



fn main(){

    input! {
        N:usize,
        A:[usize;N],
    }

    const v:usize = 10;
    let mut kari = [0;200005];
    let mut B = [0;200005];
    let mut count = 0;
    let mut flag = 0;
    kari[0] = A[0];
    B[A[0]] = A[0];
    let mut karinum:usize = 0;

    for i in 1..N+1{
        karinum = A[kari[i-1]-1];
        if B[karinum]>0 {
            kari[i]= A[kari[i-1]-1];
            for j in 0..N{
                if kari[j] == karinum{
                    flag +=1;
                }

                if flag == 1 {
                    count+=1;
                }
            }
            print!("{}\n",count);
            flag = 0;
            for j in 0..N{
                if kari[j] == kari[i]{
                    flag +=1;
                }

                if flag == 1{
                    print!("{} ",kari[j]);
                }
            }

            break;
        } 
        kari[i]= A[kari[i-1]-1];
        B[A[kari[i-1]-1]] = A[kari[i-1]-1];
    }

}