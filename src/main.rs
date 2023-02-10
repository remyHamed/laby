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

fn check_hash(entree: MD5HashCashInput, sortie: MD5HashCashOutput, index: u64) {
    let sortie = sortie.clone();
    let mut seed = sortie.seed + index * 100_000;
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
            check_hash(entree, sortie, i as u64);
        });
        children.push(child);
    }

    for child in children {
        let _ = child.join();
    }
}
