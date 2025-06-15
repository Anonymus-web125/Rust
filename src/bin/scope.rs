//Scope is the range of the program for which the item or object is valid
fn main(){
    let x: i32 = 4;
    {
        let y: i32 = 8;
        println!("The value of x is {} and the value of y is {}.",x,y);
    }
    //println!("The value of x is {} and the value of y is {}",x,y); Invalid because y is out of scope here
    println!("The value of x is {}.",x);
}