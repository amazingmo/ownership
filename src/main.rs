
use std::io::Write;
use std::rc::Rc;
/// use std::rc::Arc; also works - produces an Atomic RC ptr that is safe across threads, but slower.
/// use std::rc::Weak; also works - used for breaking links in cycles of ref counted ptrs.

fn main() {
    println!("Hello, world!");
    indexed_content();
    copy_types();
    reference_counted_ptrs();
}


fn indexed_content() -> ()
{
    let mut v = Vec::new();
    
    // Setting up
    for i in 0..5
    {
        v.push(i.to_string());
    }
    
    // It would be illegal to say
    // let mut val = v[0];
    // because then the vector will have lost ownership
    // of the value at index 0 (where 0 is not the last value.)
    
    // But there are three ways around this...
    // First, you can pop from the end...
    let fifth = v.pop().unwrap();
    assert_eq!(fifth, "4");
    
    // Second, you can swap the item of interest with the last one,
    // and pop the new last one
    let second = v.swap_remove(1);
    assert_eq!(second, "1");
    
    // Third, you can replace a value of interest with a new value.
    let first = std::mem::replace( &mut v[0], "replacement".to_string());
    
    println!("At the end, v is {:?}", &v);
    
    // Iterators allow you to consume elements of a Vector in a for loop
    
    for mut s in v
    {
        s.push('!');
        println!("{}", s);
    }
    // But v is completely unitialised now, because the loop owned it.


    // It might be that you really wanted  a vector of Option<T>
    // Then it is valid to replace an element that is Some<value>
    // with None.
    
    let mut v2 = Vec::new();
    
    for i in 0..5
    {
        v2.push(Some(i.to_string()));
    }
    
    let f2 = std::mem::replace( &mut v2[0], None );
    // Type Option has a method "take" that is shorthand for this.
    let s2 = v2[1].take();
    println!("{:?}, {:?}, {:?}", f2, s2, v2);
    
    
}

// Copy types

// Structs are not copy by default. You can turn them into a copy type by putting the
// attribute derive(Copy, Clone) on the type
#[derive(Copy, Clone)]
struct Label { number : u32 }

// To illustrate the point, use a function that moves the type (doesn't borrow)
fn print(l: Label)
{
   println!("STAMP: {}", l.number);
}

fn copy_types() -> ()
{
   let l = Label{number : 3};
   // Note that print would move the contents of l if it weren't a copy type.
   print(l);
   // and l would have been unintialised here, but for derive(Copy, Clone)
   println!("The number is {}", l.number);
}

// Reference counted ptrs

fn reference_counted_ptrs() -> ()
{
   let s = Rc::new("a string slice".to_string());
   {
      let t = s.clone(); // points to the same string.
      // if it had been "let t = s;" then it wouldn't have worked later
      // when I tried to use s in the println macro.
      println!("The message is {}", t);
   } // dropping t here doesn't make s go away
   println!("I can still print {}", s);
   
   // Note that the contents of a reference counted pointer type is IMMUTABLE
   // and can't be made mutable. Ever.
}
