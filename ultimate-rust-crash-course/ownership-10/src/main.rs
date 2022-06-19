fn main() {
    /*
     * Each value has an owner.
     * Only one owner.
     * Value gets dropped if its owner goes out of the scope.
    */

    let s1 = String::from("abc");
    let s2 = s1;
    // println!("{}", s1); Error!
    let s3 = s2.clone();
    println!("{}", s3);
    /*
     * Stack        -        Heap
     * In order     -      Unordered 
     * fixed-size   -      Variable-size
     * LIFO         -      Unordered
     * Fast         -      Slow
    */


    /*
     *   Stack              Heap
     *    s2
     *    ptr ------------>  a
     *    len 3              b
     *    capacity 3         b
     *
     *    s3
     *    ptr ------------>  a
     *    len 3              b
     *    capacity 3         c
     * 
     *    Drop:
     *    1.- Destructor.
     *    2.- Free heap.
     *    3.- Pop stack.
    */

    /*
    let s4 = String::from("abc");
    do_stuff(s4);
    println!("{}", s4); Error!

    fn do_stuff(s: String) {
        Do something.
    }

    Work but there is a better way.
    let mut s4 = String::from("abc");
    s4 = do_stuff(s4);
    println!("{}", s4); Error!

    fn do_stuff(s: String) -> String {
        Do something.
    }
    */
}

