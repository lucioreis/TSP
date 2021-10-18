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
        write!(f, "Dist:{} pela rota {}", self.x, (|vector: &Vec<usize>| {
            if vector.len() == 0 {
                String::from("[]")
            } else {
                let mut s = format!("[{}->",vector[0]);
                for i in vector {
                    s = format!("{}->{}",s,i);
                }
                return format!("{}]", s);
            }
        })(&self.v))
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
    println!("{:?}", result);
    return Ok((result.remove(0), result));
}

fn delta(rota: &Vec<usize>, distances: &Vec<i32>, n: usize) -> i32 {
    let mut delta: i32 = 0;
    for i in 0..rota.len() {//[1 2 3 4] [1.2.3.4|5.6.7.8|9.10.11.12]
        let x = (rota[i]-1)*n;
        let y = rota[(i+1)%n] -1;
        let next = x+y;
        delta = delta + distances[next];
    }//5 + 8 + 9 + 7  // [0 4 8 12][0 1 2 3]
   //[2,1,3,4]-0 7 8 7 | 5 0 8 8 | 9 6 0 9 | 10 7 10 0
    return delta;
}
#[warn(non_snake_case)]
fn calcula(n : usize, distances: Vec<i32>) -> Deck {
    //c is an encoding of the stack state. c[k] encodes the for-loop counter for when generate(k - 1, A) is called
    let mut c = std::vec!();
    let mut A = std::vec!();
    let mut result = Deck::new();

    for i in 1..=n {
        c.push(0);
        A.push(i);
    }
    result.push(delta(&A, &distances, n), &A);

    let mut counter = 1;
    let mut goal = 1;
    for i in 2..=n {goal *= i;}
    
    //i acts similarly to the stack pointer
    let mut i = 1;
    while i < n  {
        if  c[i] < i {
            if i % 2 == 0 {
                A.swap(0 as usize, i);
            }
            else {
                A.swap(c[i], i);
            }
            //println!("{}-{:?}",delta(&A, &distances, n), &A);
            counter += 1;
            if counter % 100000 == 0 {println!("{}/{}",(counter/goal)*100, 100);}
            result.push(delta(&A, &distances, n), &A);
            //Swap has occurred ending the for-loop. Simulate the increment of the for-loop counter
            c[i] += 1;
            //Simulate recursive call reaching the base case by bringing the pointer to the base case analog in the array
            i = 1;
        }
        else{
            //Calling generate(i+1, A) has ended as the for-loop terminated. Reset the state and simulate popping the stack by incrementing the pointer.
            c[i] = 0;
            i += 1;
        }
    }
    return result;
}

fn main() {
    if let Ok((num_cities, distances)) = read_data("data_matrix.txt"){
        let result = calcula(num_cities as usize, distances);
        println!("{}", result);
    }
    
}