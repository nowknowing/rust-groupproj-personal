fn main() {
    let mut x = 0;
    let mut y = &mut x;
    let mut z = &mut y;

    **z = 2;
    //1) x = 2; OR print x // can only use x. cannot y, cannot z.
    //2) *y = 2; OR print y// can only use y and x.
    //3) **z = 2; OR print z// can use z, y, x.
    println!("{}", z);

    let mut y_prime = &mut x;
    *y_prime = 4;
    
    //z = &mut y_prime;
    //**z = 5;
    println!("{}", z);
}

/*
error[E0499]: cannot borrow `x` as mutable more than once at a time
  --> borrow_chain_error.rs:12:23
   |
3  |     let mut y = &mut x;
   |                 ------ first mutable borrow occurs here
...
12 |     let mut y_prime = &mut x;
   |                       ^^^^^^ second mutable borrow occurs here
13 |     **z = 3;
   |     ------- first borrow later used here

error: aborting due to previous error; 2 warnings emitted
*/