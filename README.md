# CCDB-SCRIPT  
  
Why ccdb-script?  
  
A light language for rust when you want your game or project to easily be moddable.
  
## ccdb-script is light  
  
as said ccdb-script is made to be light  

## ccdb-script is easy
  
ccdb-script is made to be easy to integrate into any given project  
  
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

    // if you want custom values when running the script;
    let mut var = ccdb_script::var::Var::new();
    var.new_var("name","value");
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
[(drop)test]

[(if stop)]

```
  
# Uses  
  
This language is made to be easily integrated in ccdb  
and other projects  
