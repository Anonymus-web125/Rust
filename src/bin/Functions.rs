//Functions can be returning, non returning, parameterized or non parameterized
fn main(){
    parameterized_non_returning(4, 6);
    let res = parameterized_returning(4, 6);
    println!("Sum of 4, 6 is {}",res);
    let pi: f64 = non_parmeterized_returning();
    println!("The value of pi is {}",pi);
    non_parameterized_non_returning();
}
fn parameterized_non_returning(a: i32, b: i32){
    println!("{} + {} = {}",a,b,a+b);
}
fn parameterized_returning(a: i32,b: i32) -> i32{
    a+b
}
fn non_parmeterized_returning() -> f64{
    3.14159
}
fn non_parameterized_non_returning(){
    println!("Hello World!!");
}