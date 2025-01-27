fn main() {
    let mut x = 5;
    {  //This is creating a scope 
        let y = &mut x; 
        *y = 10; 
    }
    let z = &mut x; 
    *z = 15; 
}