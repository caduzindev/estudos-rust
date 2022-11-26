// example 1
// Emitirá um erro pois o tempo de vida do "x" acaba
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}                         // ---------+


// Correto
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+

// example 2
/*
Rust não pode dizer se a referência que está sendo retornada refere-se a "x" ou "y". Na verdade, também não sabemos, porque o if bloco no corpo dessa função retorna uma referência a "x" e o else bloco retorna uma referência a "y"!
*/
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Correto
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
