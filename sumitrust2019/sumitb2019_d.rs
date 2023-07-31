use proconio::{input, marker::Chars};


fn main() {
    
    input!{
        N:usize,
        S:Chars,
    }

    let mut count = 0;
    for i in 0..1000{
        let i:Vec<char> = format!("{:0>3}", i).chars().collect();
        let mut flag = 0;
        //if s == "133"{
        //    println!("a");
        //}
        for j in &S{
            //println!("{:?}",S.chars().nth(j));
            //println!("{:?}",s.chars().nth(0));
            
            if *j == i[2] && flag==2{
                flag = 3;
                continue;
            }
            if *j == i[1] && flag==1{
                flag = 2;
                continue;
            }
            if *j == i[0] &&flag==0{
                flag = 1;
                continue;
            }
            
            
        }
        if flag == 3{

            count+=1;
        }
        
    }
    println!("{}",count);

}

