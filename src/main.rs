use std::collections::HashMap;

fn hexstr_base64str(hexstr: &str) {
    let decoded = hex::decode(hexstr).expect("Decoding failed");
    let encoded = base64::encode(decoded);
    println!("{:?}", encoded);
}

fn hexstr_xor(hexstr1: &str, hexstr2: &str) {
    let decoded1 = hex::decode(hexstr1).expect("Decoding failed");
    let decoded2 = hex::decode(hexstr2).expect("Decoding failed");
    let mut xord: Vec<u8> = Vec::new();
    let mut i = 0;
    let hexstr1_len = decoded1.len();
    while i < hexstr1_len {
        xord.push(decoded1[i] ^ decoded2[i]);
        i += 1;
    }
    let encoded = hex::encode(xord);
    println!("{:?}", encoded);
}

fn xor(str: &Vec<u8>, key: &Vec<u8>) -> Vec<u8> {
    let str_len: usize = str.len();
    let key_len: usize = key.len();
    let mut i: usize = 0;
    let mut xord: Vec<u8> = Vec::new();
    while i < str_len {
        let key_index = i % key_len;
        xord.push(str[i] ^ key[key_index]);
        i += 1;
    }
    xord
}

fn english_probability(ascii_byte: usize) -> f64 {
    let english_letter_probability: [f64; 26] = [
        0.08497,
        0.01492,
        0.02202,
        0.04253,
        0.11162,
        0.02228,
        0.02015,
        0.06094,
        0.07546,
        0.00153,
        0.01292,
        0.04025,
        0.02406,
        0.06749,
        0.07507,
        0.01929,
        0.00095,
        0.07587,
        0.06327,
        0.09356,
        0.02758,
        0.00978,
        0.02560,
        0.00150,
        0.01994,
        0.00077
    ];
    let mut probability = 0.0;
    if ascii_byte >= 65 && ascii_byte <= 90 {
        // uppercase letter
        probability = english_letter_probability[ascii_byte - 65];
    } else if ascii_byte >= 97 && ascii_byte <= 122 {
        // lowercase letter
        probability = english_letter_probability[ascii_byte - 97];
    }
    probability
}

fn chal3(hexstr: &str) {

    let decoded = hex::decode(hexstr).expect("Decoding failed");

    let mut scoreboard: HashMap<u8, f64> = HashMap::new();

    for n in 65..90 {
        let xor_key = vec![n];
        let xor_str = xor(&decoded, &xor_key);
        let mut string_probability: HashMap<u8, f64> = HashMap::new();

        for char in &xor_str {
            if !string_probability.contains_key(&char) {
                let count = xor_str.iter().filter(|&x| *x == *char).count();
                let length = xor_str.len();
                let probability: f64 = count as f64 / length as f64;
                string_probability.insert(*char, probability);
            }
        }
        
        let mut score: f64 = 0.0;

        for (char, probability) in string_probability {
            let english_probability = english_probability(char.into());
            score += (probability * english_probability).sqrt();
        }
        scoreboard.insert(n, score);
    }

    let mut highest = 0;
    let mut highest_score = 0.0;
    for (char, score) in scoreboard {
        if score > highest_score {
            highest = char;
            highest_score = score;
        }
    }
    println!("Highest scoring dec is {} with {}", highest, highest_score);
}

fn main() {
    /*
        challenge 1
    */
    let chal1input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    hexstr_base64str(chal1input);
    /*
        challenge 2
    */
    let chal2input1 = "1c0111001f010100061a024b53535009181c";
    let chal2input2 = "686974207468652062756c6c277320657965";
    hexstr_xor(chal2input1, chal2input2);

    /*
        challenge 3
    */
    let chal3input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    chal3(chal3input);
}
