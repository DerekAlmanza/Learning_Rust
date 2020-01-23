# Learning Rust

A repository created for learn and review the basic concepts for Rust. This code is fully documented.

* Hello. Basic lessons:
  * Write and run a Hello, world! program using rustc directly
  * Create and run a new project using the conventions of Cargo
  
***
  
* Guess Game. Basic lessons:

  * Introduce you to many new Rust concepts like:
    * let
    * match
  * And the basic use of:
    * Methods
    * Associated functions
    * External crates
    
***
 
* Common Programming Concepts.
 
  In this lesson we learn the general programming concepts like:
    * Variables and mutability: There are two kinds of variables; 
      * Inmutable variables: They are declared as `let`.
      * Mutable variables: They are declared as `let mut` and their value will can change.
    * Constants: The constants never change their value.
    * Shadowing: Use the same variable name repeatdly for different uses. 
    * Data types: To name a few types are;
      * Integer types.
      * Float types.
      * Numeric operations.
      * Boolean types.
      * Char types.
    * Functions.
    * Control flows: We decide wether run or not run our code depending of circunstances, the most common structures are:
      * If: An if expression allows you branch your code depending on conditions.
      * Loop: The loop keyword tells Rust to execute a block of code over and over again forever until you explicitly tell    it to stop.
      * While: It’s often useful for a program to evaluate a condition within a loop. While the condition is true, the loop runs.
      * For: It's useful to loop over the elements of a collection. 
       
***

* Ownership.
  
  We learned the ownership concept,as well the borrowing and the slices cconcept. These concepts guarantee the security of our program. The details of concepts be in the file completely in Spanish with the name of `ownership.md`.    
  
***
  
* Structs.

  In this lesson we learned the strcuts definition; To   define a struct, we put in the keyboard `struct` and the  name of     structure. Inside of the curly brackets, we  define the names and the data types of the pieces of data.   Let's call this as  *fields*.
  We also learned:
  * Print the fields of structs with `#[derive(Debug)]` syntax.
  * Other type of structs; __Tuple Structures__. It's a simpler way for build structs, without a name variables, only the data type and how many attributes you wish in struct.
  * Modifies the data struct in a variable.
  
***
* Methods.

  Methods let you specify the behavior that instances of   your structs have, and associated functions let you   namespace functionality that is particular to your struct without having an instance available.
  
   
  
