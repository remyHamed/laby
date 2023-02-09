use std::thread;

#[derive(Clone)]
struct MD5HashCashInput {
    // complexité en bits
    complexity: u32,
    // message à signer
    message: String,
}

#[derive(Clone)]
struct MD5HashCashOutput {
    // Graine utilisée pour résoudre le défi
    seed: u64,
    // code de hash trouvé en utilisant seed + message
    hashcode: String,
}

fn main() {
    let entree = MD5HashCashInput {
        complexity: 9,
        message: String::from("hello"),
    };

    let sortie = MD5HashCashOutput{
        seed: 844,
        hashcode: String::from("x"),
    };

    let mut children = vec![];

    for i in 0..4 {
        let entree = entree.clone();
        let sortie = sortie.clone();
        let child = thread::spawn(move || {
            let sortie = sortie.clone();
            let mut seed = sortie.seed + i as u64 * 100_000;
            for _ in 0..100_000 {
                let mix = concat(entree.message.clone(), seed);
                let hashcode = hashmd5(&mix);
    
                let mut count = 0;
                for c in hashcode.chars() {
                    if c == '0' {
                        count += 4;
                    } else {
                        if c == '1'{
                            count += 3;
                        }
                        if c == '2' || c == '3'{
                            count += 2;
                        }
                        if c >'3' && c < '8'{
                            count += 1;
                        }
                        break;
                    }
                }
                if count >= entree.complexity {
                    println!("Hash trouvé: {} avec seed: {}", hashcode, seed);
                    break;
                }
                seed += 1;
            }
        });
        children.push(child);
    }
    

    for child in children {
        let _ = child.join();
    }
}


fn hashmd5(word: &String) -> String{
    let digest = md5::compute(word);
    let hash = digest.iter().map(|x| format!("{:02x}", x)).collect::<String>();
    return hash;
}

fn concat(message: String, seed: u64) -> String{
    let seedInHex = format!("{:016X}", seed);
    let concat = format!("{}{}", seedInHex, message);
    return concat;
}


/*use std::thread;

#[derive(Clone)]
struct MD5HashCashInput {
    // complexity in bits
    complexity: u32,
    // message to sign
    message: String,
}

#[derive(Clone)]
struct MD5HashCashOutput {
    // Seed used to solve the challenge
    seed: u64,
    // hashcode found using seed + message
    hashcode: String,
}

fn main() {
    let entrer = MD5HashCashInput {
        complexity: 9,
        message: String::from("hello"),
    };

    let mut sortie = MD5HashCashOutput{
        seed: 6666,
        hashcode: String::from("x"),
    };

    let mut children = vec![];

    for i in 0..4 {
        let entrer = entrer.clone();
        let child = thread::spawn(move || {
            let mut sortie = sortie.clone();
            sortie.seed = sortie.seed + i as u64 * 100_000;
            for seed in sortie.seed..sortie.seed + 100_000 {
                let mix = concat(entrer.message.clone(), seed);
                let hashcode = hashmd5(&mix);

                let mut count = 0;
                for c in hashcode.chars() {
                    if c == '0' {
                        count += 4;
                    } else {
                        if c == '1'{
                            count += 3;
                        }
                        if c == '2' || c == '3'{
                            count += 2;
                        }
                        if c >'3' && c < '8'{
                            count += 1;
                        }
                        break;
                    }
                }
                if count >= entrer.complexity {
                    println!("Hash trouvé: {} avec seed: {}", hashcode, seed);
                    break;
                }
            }
        });
        children.push(child);
    }

    for child in children {
        let _ = child.join();
    }
}


fn hashmd5(mut word: &String)-> String{
    let digest = md5::compute(word);
    let hash = digest.iter().map(|x| format!("{:02x}", x)).collect::<String>();
    return hash;
}

fn concat(message: String, seed: u64)-> String{
    let seedInHex = format!("{:016X}", seed);
    let concat = format!("{}{}", seedInHex, message);
    return concat;
}


use std::thread;

fn main() {
    let entrer = MD5HashCashInput {
        complexity: 9,
        message: String::from("hello"),
    };

    let mut sortie = MD5HashCashOutput{
        seed: 6666,
        hashcode: String::from("x"),
    };

    let mut children = vec![];

    for i in 0..4 {
        let entrer = entrer.clone();
        let child = thread::spawn(move || {
            let mut sortie = sortie.clone();
            sortie.seed = sortie.seed + i as u64 * 100_000;
            for seed in sortie.seed..sortie.seed + 100_000 {
                let mix = concat(entrer.message.clone(), seed);
                let hashcode = hashmd5(&mix);

                let mut count = 0;
                for c in hashcode.chars() {
                    if c == '0' {
                        count += 4;
                    } else {
                        if c == '1'{
                            count += 3;
                        }
                        if c == '2' || c == '3'{
                            count += 2;
                        }
                        if c >'3' && c < '8'{
                            count += 1;
                        }
                        break;
                    }
                }
                if count >= entrer.complexity {
                    println!("Hash trouvé: {} avec seed: {}", hashcode, seed);
                    break;
*/