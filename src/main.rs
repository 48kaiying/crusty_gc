// struct Header {
//     size : u64, // allocation size in bytes
//     next : Header   
// }

fn main() {
    let mut x = 5; 
    println!("the value of x is {x}");
    x = 4;
    println!("the value of x is {x}");
    
    {
        let x = 100; 
        println!("This is an example of shadowing {x}");

    }

    println!("Hello, world!");

    // let f = Header {
    //     size = 48
        
    // }
}
