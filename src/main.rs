extern crate rand;
use rand::Rng;

fn main() {
    let tmp = consume("Now is not the time for desert, now is the time for dinner".to_string());
    println!("~~~~~");
    produce(&tmp);
}

fn consume(txt: String) -> Vec<(String, String, Vec<String>)> {
    let words : Vec<String> = txt.split_whitespace().map(|x| x.to_string()).collect();
    let mut data : Vec<(String, String, Vec<String>)> = vec![];
    for slice in words.windows(3) {
        match data.iter().position(|ref x| x.0 == slice[0] && x.1 == slice[1]) {
            Some(x) => if !data[x].2.contains(&slice[2]) {data[x].2.push(slice[2].clone());},
            None => data.push((slice[0].clone(), slice[1].clone(), vec!(slice[2].clone())))
        }
    }

    for tuple in data.iter() {
        println!("{:?}", tuple);
    }
    data
}

fn produce(data : &Vec<(String, String, Vec<String>)>) {
    let mut tmp = &data[0];
    print!("{} {}", tmp.0, tmp.1);
    loop {
        let mut tuple = (tmp.0.clone(), tmp.1.clone());
        let mut word = rand::thread_rng().choose(&tmp.2)
                                         .unwrap()
                                         .clone();
        print!(" {}", word);
        tuple.0 = tuple.1;
        tuple.1 = word;
        match data.iter().find(|x| x.0 == tuple.0 && x.1 == tuple.1) {
            Some(x) => tmp = x,
            None => break
        }
    }
}
