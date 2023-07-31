use proconio::input;


fn main() {
    input!{
        a:usize,
        b:usize,
        c:usize,
        x:usize,
        y:usize,
    }
    let mut min = 5000*1000000;
    let mut num = 0;

    for i in 0..1000000{
        num = 0;
        if(i%2 != 0){
            continue;
        }
        num+= i*c;
        if(x>i/2){
            num+= a*(x-i/2);
        }
        if(y>i/2){
            num+= b*(y-i/2);
        }

        if(min>num){
            min = num;
        }
    }
    println!("{}",min);
}
