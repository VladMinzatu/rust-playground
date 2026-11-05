struct MyStruct<'a> {
    field1: &'a i32,
    field2: &'a String, // same lifetime as field1
}

fn main() {
    let x = 10;
    let r: &i32;
    {
        let s = String::from("Hello, world!");
        {
            let my_struct = MyStruct { field1: &x, field2: &s };
            r = my_struct.field1; // r is a reference to x, which is valid
            
            let r2 = my_struct.field2; // r2 is a reference to s, which is valid
            println!("r2 = {}", r2); // This is fine because r2 is a reference to s, which is still valid
        }
        println!("r = {}", r); // This is fine because r is a reference to x, which is still valid
    }
    // println!("r = {}", r);  - Problem! Both fields of my_struct have the same lifetime, so we need to find a lifetime that works for both. But s is dropped at the end of the inner scope, so we can't use my_struct.field2 after that point. This would cause a compile error.
}