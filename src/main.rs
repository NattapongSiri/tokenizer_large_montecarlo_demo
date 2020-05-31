use std::collections::{HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

/// This function create a new dictionary file by taking original file and split
/// all dictionary entry if it contain whitespace into separate entry.
/// It using HashSet to store a dictionary entry to prevent duplication before flusing it
/// to target file.
fn clean_dict<P: AsRef<std::path::Path>>(source: P, target: P) -> std::io::Result<HashSet<String>> {
    let reader = BufReader::new(File::open(source)?);

    let mut dict = HashSet::new();

    reader.lines().for_each(|line| {
        line.unwrap().split_whitespace().for_each(|word| {
            dict.insert(word.to_owned());
        });
    });
    let mut writer = BufWriter::new(File::create(target)?);
    dict.iter().for_each(|word| {
        writer.write(word.as_bytes()).unwrap();
        writer.write(b"\n").unwrap();
    });
    Ok(dict)
}

fn main() {
    use rand::seq::SliceRandom;
    use std::time::{Instant};
    use tokenizer::Tokenizer;
    use tokenizer::th;

    let original_dict_path = "data/lexitron_utf8.txt";
    let clean_dict_path = "data/lexitron_mod.txt";
    let dict = clean_dict(original_dict_path, clean_dict_path).unwrap();
    let mut words: Vec<String> = dict.into_iter().collect(); // turn hashset into vec
    let mut rng = rand::thread_rng();
    let mut mean_f1 = 0f64;
    let mut var_f1 = 0f64;
    let mut best_f1 = 0f64;
    let mut worst_f1 = 1f64;
    let times = 100; // run montecarlo simulation for 10 times
    let unknown_count = (words.len() as f64 * 0.1) as usize; // 10% of dictionary

    for k in 0..times {
        let mut true_positive = 0;
        
        words.shuffle(&mut rng);
        let tok_dic = &words[unknown_count..];

        println!("Tokenization dictionary size is {}", tok_dic.len());
        println!("Total unknown word in mix {}", words.len() - tok_dic.len());

        // need to reshuffle again to prevent a continguous large series of unknown owrd
        let mut test_words = words.clone();
        test_words.shuffle(&mut rng);
        // construct a test text
        let test_text = test_words.iter().fold("".to_owned(), |mut v, w| {v.push_str(w); v});

        // construct an expected slice for fast comparison
        let mut byte_count = 0;
        let expected: Vec<&str> = test_words.iter().map(|w| {
            let cur_count = byte_count;
            byte_count = cur_count + w.len();
            &test_text[cur_count..byte_count]
        }).collect();

        // measure time it take 
        let instantiate_time = Instant::now();
        let tokenizer = th::Tokenizer::from(tok_dic);
        println!("Simulation {} has total tokenizer instantiate time {} ms", k, instantiate_time.elapsed().as_millis());
        // let dict = tokenizer::dict::SizedDict::from(words);

        let begin = Instant::now();

        let tokens = tokenizer.tokenize(&test_text);
        
        println!("Simulation {} tokenization is done in {} ms", k, begin.elapsed().as_millis());

        let actual_positive = expected.len();
        let predicted_positive = tokens.len();

        let mut i = 0;
        let mut byte_i = 0;
        let mut j = 0;
        let mut byte_j = 0;

        while i < tokens.len() && j < expected.len() {
            if std::ptr::eq(tokens[i], expected[j]) { 
                // compare by slice attribute rather than entire str comparison
                true_positive += 1;
                byte_i += tokens[i].len();
                i += 1;
                j += 1;
                byte_j = byte_i;
            } else if byte_i < byte_j { // `i` position in test_text is lack behind `j`
                byte_i += tokens[i].len();
                i += 1;
            } else if byte_j < byte_i { // `j` position in test_text is lack behind `i`
                byte_j += expected[j].len();
                j += 1;
            } else { // case where both i and j are in the same position in test_text
                byte_i += tokens[i].len();
                i += 1; 
                byte_j += expected[j].len(); // bytes len is dif so ptr::eq doesn't match
                j += 1;
            }
        }

        let processed_time = begin.elapsed().as_millis();
        let precision = (true_positive as f64) / (predicted_positive as f64);
        let recall = (true_positive as f64) / (actual_positive as f64);
        
        let f1_score = 2f64 * (precision * recall) / (precision + recall);
        let prev_mean = mean_f1;
        if k > 0 {
            // k is 0 based while in math formula k is 1 based. So the formula is adjusted to reflex this.
            mean_f1 = ((mean_f1 * k as f64) + f1_score) / (k + 1) as f64; 
        } else {
            mean_f1 = f1_score;
        }

        if k > 1 {
            // k is 0 based while in math formula k must be 1 based. So the formula is adjusted to reflex this.
            var_f1 = ((k - 1) as f64 * var_f1 + k as f64 * (prev_mean - mean_f1).powi(2) + (f1_score - mean_f1).powi(2)) / k as f64;
        }

        if f1_score > best_f1 {
            best_f1 = f1_score;
        }

        if f1_score < worst_f1 {
            worst_f1 = f1_score;
        }

        println!("Simulation {} got F1 score = {}", k, f1_score);
        println!("Simulation {} take total processing time = {} m {} s {} ms", k, processed_time / 60_000, (processed_time / 1000) % 60, processed_time % 1000);
    }

    println!("Average F1 score = {}", mean_f1);
    println!("F1 variance = {}", var_f1);
    println!("Best F1 score = {}", best_f1);
    println!("Worst F1 score = {}", worst_f1);
    println!("Margin of error at 95% for F1 = {}", (1.984 * var_f1 / ((times - 1) as f64).powf(0.5)));
    println!("Margin of error at 99% for F1 = {}", (3.391 * var_f1 / ((times - 1) as f64).powf(0.5)));
}