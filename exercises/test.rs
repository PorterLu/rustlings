fn main(){
    let a: Vec<i32> = (1..).filter(|x| x%2==0).take(5).collect();
    println!("{:?}", a);
}
