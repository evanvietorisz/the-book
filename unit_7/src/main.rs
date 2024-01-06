fn main() {
    /*

    Packages, Crates, and Modules

    a crate is a piece of code considered by the compiler. it's a name
    for a library. crates can be binary crates, or library crates. 
    
    binary crates are compiled into executables. they must have a main()
    function that defines what happens when the executable runs

    library crates dont have a main function and don't compile into
    an executable. they define functionality intended to be shared
    with multiple projects

    example is the rand crate, which is a library of functionality
    for generating random numbers

    crates have a crate root, which is a source file that is the root
    module of the crate

    a package is a set of multiple crates. it contains a Cargo.toml
    file that describes how to build the crates 

    a package can contain many binary crates but only one library crate

    when you do cargo new, it's creating a new package

    cargo new my_package yields a my_package/ directory with
    Cargo.toml
    src
        -main.rs
    
    the Cargo.toml file has no mention of the main.rs or anything else
    that's because the convention is that the main.rs file defines
    a binary crate with the same name as the package. likewise, the
    lib.rs file, if it exists, defines a library crate of the same name
    as the package 
    
    if a package contains both main.rs and lib.rs, it has two crates

    a package can have multiple binary crates inside a src/bin directory


    ### Modules ###

    a module is like a subdirectory of a crate that contains functions

    when you make a package, it automatically makes a main.rs file that
    implicitly creates a crate by the name of the package. that is the root
    module of that crate. inside that file, you can declare/define a tree
    structure of modules by using mod module_name;

    when you do that, the compiler will look for the code referred to 
    by that name in three places:
    - right there in the file itself, such as mod my_module {
        ....
    }
    - a file inside the folder inside a folder src/my_module.rs
    - a file inside the folder src/my_module/mod.rs

    when you access the path to a function or something, you use the use 
    keyword and specify a path using ::

    use implicit_name_of_crate::submodule1::submodule2::function_i_want

    this allows you to use function_i_want in the script

    to reiterate, you define the tree structure by 
    using mod module_name; in your files, even if you don't want to 
    bring what those modules contain into scope.

    note: you can use the literal crate to refer to the 
    implicit_name_of_crate in the example above



    ### importing stuff via its path ###

    we can use absolute paths beginning with the 'crate' literal
    to refer to the current crate 

    we can also build relative paths using the super literal or 
    just the name of the current module. we can also start
    from a module defined in the same crate


    in rust all modults, structs, enums, constants, etc are private
    by default, and are hidden to parent modules. you need to use
    the pub keyword before anything you want to be accessible to parent
    modules

    things in child modules CAN use things in their ancestors

    to expose a function in a child module, you need to make both the 
    module and function public

    sibling modules can refer to each other regardless of privacy

    note: good practice for structuring projects is that the entire 
    library of modules should be defined from the library crate, and 
    the binary crate should use it just like an external client would


    you make structs and enums public by adding pub before them 
    and also adding pub before all their attributes that you want to be
    public. same goes for methods inside an impl block. you can make
    things public and private on an individual basis


    ### differences between making structs and enums public ###

    all struct fields are private by defailt 
    if a struct has private fields, you need to explicitly 
    expose a public function that constructs it, since only a method 
    has the right to edit a private field

    if you make an enum public, all its variants become public


    ### use ###

    use brings something into scope

    the new name is only valid in the scope in which the use was used

    ie you cant use a use at the top of a crate, then define
    a submodule inside the crate and use the thing imported by the 
    use in it

    okay... so unresolved question is what about the idea that child 
    modules can see what their parents had? 

    an idiomatic way to use the use is that if you want a particular
    function x, then you use use to bring the function's parent
    module into scope. then when you want to call the function, you do
    parent_module::x

    this makes clear to readers of the code that the function is 
    externally defined

    when you bring structs and enums into scope, just bring in the 
    object itself
    an exception to this is if you have two objects of the same name
    rust doesnt allow this, so in this case, you would bring
    the two objects' parent modules into scope and call them
    parent_module1::object, parent_module2::object in order
    to disambiguate

    you could also resolve shared names by renaming an imported 
    object using the 'as' keyword

    like use blah::blah::blah as blah

    we can combine pub and use to make something we import 
    accessible to code that uses our code

    pub use blah::blah::Hello in module_name
    allows someone else to go 
    use module_name::Hello
    in their own code

    this feature allows you to write code with one structure and 
    expose it in a different structure

    
    ### using external packages ###

    1) add package_name = "0.8.5" ie the version name to the Cargo.toml file
    2) bring the package into scope in your package

    note: the standard library is an 'external' package, but it is
    shipped with rust. so we dont need to add it to the Cargo.toml, 
    but we do need to import things from it into a package's namespace, 
    like std::collections


    we can bring in multiple things at a time like
    use std::{cmp::Ordering, io};
    and 
    use std::io::{self, Write};

    the latter brings in std and a thing inside it

    we can also bring in all public items from a module using the 
    glob operator

    use std::collections::*;

    ### how to split up modules into different files

    put mod my_mod; in the script and then in the directory
    have my_mod.rs or /my_mod/mod.rs 

    you can have both a my_mod.rs file and my_mod/ directory
    if the rs file only has a
    pub mod sub_dir; 
    line in it


    note: the forms that use mod.rs are outdated. you can only use
    the mod.rs style if ALL paths are specified like this

    to summarize, if we have some root crate main.rs which has child
    X, the root crate will have mod X; in it.
    and there will be a file src/X.rs

    if there is a submodule of X called Y, then X will contain
    pub mod Y; 

    and there will be a file at src/X/Y.rs


































































    

































    */
}
