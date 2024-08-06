# Transpiler in Rust for JVM

```sh
cargo run --bin repl
```

## objective 1

compile for jvm 

```
    var wasd = 5; 
    var a = wasd; 
    var aa = a;   
    var aaa = wasd; 
    var aaaa = 1; 
    print(a); // 5
    print(1); // 1
    print(20); // 20
    print(aaaa); // 1
```
