mod allocator;
mod tests; 

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

    allocator::alloc_init();
    // test::basics();

    println!("Done!");
}
