use std::collections::HashMap;
use std::io;
fn main() {
    let _v: Vec<i32> = Vec::new();
    let _v2 = vec![1,2,3];

    let mut v3 = Vec::new();

    v3.push(65);
    v3.push(54);

    example1();
    example2();
    example3();
    vetor_task();
    pig_latin();
}
fn vetor_task() {
    let vetor1 = vec![1,2,3,3];
    let calc = vetor1.len() / 2;
    let middle = &vetor1[calc];

    let mut hashmap = HashMap::new();
    let mut max_element = 0;
    let mut max = 0;
    for i in &vetor1 {
        let el = hashmap.entry(i).or_insert(0);
        *el += 1;
        if *el > max {
            max_element = *i;
            max = *el;
        }
    }

    println!("O do meio {} e o maximo é {}",middle,max_element)
}

fn pig_latin() {
    println!("Enter the string to be Pig-latinized");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read from stdin");

    let s: Vec<char> = input.chars().collect();
    let mut new_s: String = String::new();
    let consonants = ['b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q',
    't', 'r', 's', 'v', 'w', 'x', 'z', 'B', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q',
    'T', 'R', 'S', 'V', 'W', 'X', 'Z'];

    let mut i = 0;

    while i < s.len() {

        if !s[i].is_alphabetic() {
            new_s = format!("{}{}",new_s,s[i]);
            i += 1;
            continue;
        }

        let first_letter = s[i];
        //let mut appendage: Vec<char> = "-hay".chars().collect();
        let mut appendage = String::new();

        if consonants.contains(&first_letter) {
            appendage = format!("-{}ay",&first_letter);
            i += 1;
        } else {
            appendage = format!("{}-hay",&first_letter);
            i += 1;
        }

        while s[i].is_alphabetic() && i < s.len() {
            new_s = format!("{}{}",new_s,s[i]);
            i += 1;
            continue;
        }

        new_s = format!("{}{}",new_s,appendage);
    }

    println!("{}", new_s);
}
fn example1() {
    let v = vec![1,2,3,4,5];

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

fn example5() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teamName = String::from("Blue");
    let score = scores.get(&teamName).copied().unwrap_or(0);

    println!("{}", score)

    for (key,value) in &score {
        println!("Chave: {}, Valor: {}",key,value)
    }
}
