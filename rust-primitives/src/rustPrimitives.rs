// Rust primitives

/*
Scalar Types
Signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
Unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
Floating point: f32, f64
char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
bool either true or false
The unit type (), whose only possible value is an empty tuple: ()
*/

fn main() {
    // Boolean variable with type annotated.
    let logical: bool = true;
    println!("boolean = {}", logical);

    // Float 64 variable with regular annotation.
    let a_float: f64 = 1.0;  
    println!("float 64 = {}", a_float);

    // Integer 32 with suffix annotation.
    let an_integer = 5i32; 
    println!("integer 32 = {}", an_integer);

    // Variable creation with default used.
    // float 64
    let default_float = 3.0; 
    println!("default float 64 = {}", default_float);

    // int 32
    let default_integer = 7;
    println!("default integer 32 = {}", default_integer);
    
    // Mutable int 32 default.
    let mut inferred_type = 12;
    println!("inferred type integer 32 = {}", inferred_type);

    // Variable types inferred from context.
    // Integer 64 is inferred from another line.
    inferred_type = 4294967296i64;
    println!("inferred by context integer 64 = {}", inferred_type);

    // Mutable variable, value can be changed.
    // Mutable i32 variable.
    let mut mutable = 12;
    println!("mutable = {}", mutable);

    mutable = 21;
    println!("changed value mutable = {}", mutable);

    // It is not allowed to change variable type.  Error.
    // mutable = true;

    // Overwriting variables (shadowing) is not allowed.
    // let mutable = true;

    /* Compound types - Array and Tuple */

    // Array of type i32 and length 5.
    let my_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array integer 32 = {:?}", my_array);

    // Tuple with different types.
    let my_tuple = (5u32, 1u8, true, -5.04f32);
    println!("tuple unsigned int 32, unsigned int 8, boolean, float 32 = {:?}", my_tuple);
}