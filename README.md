# CCDB_SCRIPT  
  
Why ccdb-script?  
  
A light language for rust when you want your game or project to easily be moddable.
  
## ccdb_script is light  
  
as said ccdb_script is made to be light  

## ccdb_script is easy  
  
ccdb_script is made to be easy to integrate into any given project  
  
## How to integrate  
  
```rust
// ! note this example uses my other crate openfile 
use ccdb_script;
/* The test ccdb-script 
[(var)test "test"]
*/
fn main(){
    ccdb_script::run(openfile::readFile("test.ccdbs")); // you can also get val from this
    let parse = ccdb_script::parse(openfile::readFile("test.ccdbs")); // if you split it up this way you only need to parse once and then 
    // you can still easily run the script whenever you want to 
    let mut val = ccdb_script::run_parsed(parse);
    println!("{}",val.get_var("test".to_string()));

    // create a custom struct. This is used for custom functions in the script that are linked to rust
    let mut custom = ccdb_script::custom::Custom::new();
    //sample function ALL custom functions need Vec<String> and -> String as their only in and outputs
  
    fn test(x: Vec<String>) -> String{
        println!("{:#?}",x);
        "test".to_string()
    }
    fn test2(x: Vec<String>){
        println!("no output{:#?}",x);
       
    }
    custom.new_fn(ccdb_script::custom::FnType::Output(test),"test");//with output 
    custom.new_fn(ccdb_script::custom::FnType::Nout(test2),"test2");//without output
// add it to the script
    // the output will be R(function name) with no ()
    // if you want custom values when running the script;
    let mut var = ccdb_script::var::Var::new();
    var.new_var("name","value");
    var.set_max_mem(100); // set the max number of bytes the script is allowed to use if you set it to 0 it will be infinite(default)
    ccdb_script::run_parsed_var(parse, &mut var);
}
```
  
# Syntax
  
```txt

[(Command)arguments]

example: 
[(var)test "test"]
[(print)test]
[(print)"Hello, world"]
[(if)test == "test"]
[(edit)test "test2"]

[(print)"new test"]

[(print)test]
[(print)"That is true"]

[(print)"dropping test"]
/ drop a variable by using drop
[(drop)test]

[(if stop)]

[(var)int "0"]
[(print)int]
/ use + / - and * like this before the number to do a math operation
[(edit)int +100]
[(print)int]
[(custom) arg1 arg2 arg3 ...]


```
  
# Uses  
  
This language is made to be easily integrated in ccdb  
and other projects  
