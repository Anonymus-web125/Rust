//Shadowing means redefining a variable with the same name 
//Here the varible with the old name is no longer valid
fn main(){
    let x = 5;
    let x = 8;
    println!("The value of x is {}",x);
}