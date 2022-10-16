fn main() {
    let _v: Vec<i32> = Vec::new();
    let _v2 = vec![1,2,3];

    let mut v3 = Vec::new();

    v3.push(65);
    v3.push(54);

    example1();
    example2();
    example3();
}

fn example1() {
    let v =  vec![1,2,3,4,5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(value) => println!("The third element is {}", value),
        None => println!("The third element not found"),
    }
}

fn example2() {
    let v = vec![1,2,3];
    for i in &v {
        println!("{}",i);
    }

    let mut v2 = vec![1,2,3];
    for i in &mut v2 {
        *i += 50;
    }
}
enum Teste {
    Int(i32),
    Float(f64),
    Text(String),
}
fn example3() {
    let mut v: Vec<Teste> = Vec::new();

    v.push(Teste::Int(25));
    v.push(Teste::Float(25.50));
    v.push(Teste::Text(String::from("ABC")));
}

fn example4() {
    let mut s1 = String::from("Car");
    s1.push_str("los");
    println!(s1);
}
