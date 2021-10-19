use std::fs::File;
use std::io;
use std::path::Path;
use std::io::Read;
use std::string::String;
use std::fmt;

enum Erro {
    FileError(io::Error),
}

struct Deck {
    x: i32,
    v: Vec<usize>,
}

impl Deck {
    fn new() -> Deck { 
        Deck {x: i32::MAX, v: Vec::<usize>::new()}
    }
    fn push(&mut self, value: i32, v: &Vec<usize>) {
        if value < self.x {
            self.x = value;
            self.v = v.clone();
        }
    }
}

impl std::fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Dist:{} pela rota {:?}", self.x, self.v)
    }
}

fn read_data(filename: &str) -> Result<(i32, Vec::<i32>), Erro> {
    let mut result = vec![];
    let long_file_name = format!("src/assets/{}", filename);
    let path = Path::new(&long_file_name);
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(why) => return Err(Erro::FileError(why)),
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read: {}", why),
        Ok(_) => print!("Read file done!\n"),
    }
    for cities in s.split("\n").collect::<Vec<&str>>() {
        for city in cities.split(" ") {
            if let Ok(c) = city.parse::<i32>() {
                result.push(c);
            }
        }
    }
    return Ok((result.remove(0), result));
}

fn delta(rota: &Vec<usize>, distances: &Vec<i32>, n: usize) -> i32 {
    let mut delta: i32 = 0;
    for i in 0..rota.len() {
        let x = (rota[i]-1)*n;
        let y = rota[(i+1)%n] -1;
        let next = x+y;
        delta = delta + distances[next];
    }
    return delta;
}
#[allow(non_snake_case)]
fn calcula(n : usize, distances: Vec<i32>) -> Deck {
    let mut c = std::vec!();
    let mut A = std::vec!();
    let mut result = Deck::new();

    for i in 1..=n {
        c.push(0);
        A.push(i);
    }
    result.push(delta(&A, &distances, n), &A);

    let mut i = 1;
    while i < n  {
        if  c[i] < i {
            if i % 2 == 0 {
                A.swap(0 as usize, i);
            }
            else {
                A.swap(c[i], i);
            }
            result.push(delta(&A, &distances, n), &A);
            c[i] += 1;
            i = 1;
        }
        else{
            c[i] = 0;
            i += 1;
        }
    }
    return result;
}

fn main() {
    println!("---Dados fornecidos pelo trabalho---");
    if let Ok((num_cities, distances)) = read_data("data_13.txt"){
        let result = calcula(num_cities as usize, distances);
        println!("{}", result);
    }
    
    println!("---Dados coletados na internet---");
    if let Ok((num_cities, distances)) = read_data("data_2.txt"){
        let result = calcula(num_cities as usize, distances);
        println!("{}", result);
        let nomes = vec!["VC","BH","RJ","SP","VT","CU","FL","PA"];
        println!("{}", "For Humans:");
        println!("Distâcia: {}KM\nPela rota: {:?}", result.x, (|vector: &Vec<_>|{
            let mut s = String::new();
            for i in vector {
                s.push_str(nomes[*i-1]);
                s.push_str(" ");
            }
            s
        })(&result.v));
        
    }
    
}