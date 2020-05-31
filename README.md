# tokenizer_demo
A demo on how to use crate tokenizer to tokenize Thai text.

## Background
Thai language is a language used in Thailand. It is one of "Complex Text Language".
What make it complex is that it has no space between words. It has no clear rule on word boundary. It use various non-spacing mark characters. Name of entity usually comprise of regular Thai word. It is recommended to have space between each sentence but this rule is not strictly enforced. Thai language usually come mixing with borrowed word from foreign languages. Sometime these words come in romanization form. Despite the fact that Royal Institute of Thailand has a clear define rule for how to romanize Thai text, it is strictly adopt only in formal communication area such as News or academic research area.

## How to identify Thai word boundary ?
This is an actively research area for many decades. The early ear, researcher found that using Dictionary yield acceptable word break result. Later on, they start using other feature such as part-of-speech, rule-based, etc. It improve some specific case with specific constraints, e.g. no unknown word in text, no word ambiguity. In recent year, many research use deep learning to break Thai word. It show promising result with new kind of restriction, e.g. different word came out from different sentence even if it is exactly the same word. This is due to side effect of context around such word while training which may dominance the expected result. In practical use, Thai people use hybrid approach. Dictionary is still an essential part on every hybrid approach. This is because dictionary based is easier to maintain. It is easier to let user define new word.

## Word tokenizer in Rust
This repository demonstrate how to use `tokenizer` crate to tokenize Thai word by using Lexitron dictionary.
Lexitron is a dictionary built from NECTEC. See `data/lexitron-license` for licensing info of the dictionary.

See `src/main.rs` for complete code.
In there, there's two functions. `clean_dict` and `main`.

`clean_dict` function will break an entry that have space inside the entry into multiple entries. This function is needed because Lexitron dict entry contains multiple words per entry. 

`main` is where the program  entry is.

## Hyper parameters choice
- times - A number of simulation to perform. In this repo, I choose 100.
- unknown_count - A number of words to exclude from tokenizer dictionary, I choose 10% of dict.

## Test setup
On each simulation:
1. Shuffle entire dictionary
1. Take a slice of dictionary starting from unknown_count to the rest of dictinoary.
1. Create tokenizer using the slice in previous step
1. Shuffle entire dictionary
1. Concatenate entire dictionary into single long text
1. Feed concatenated text to tokenizer
1. Verify tokenization result agains the last shuffle dictionary entry
1. Take F1-score
1. Take global average F1-score
1. Take global variance of F1-score

We also print a time it take on following step:
1. Tokenizer instantiation step, 
1. Tokenization
1. Total time since instantiation until all tokens are verified.
1. Final score as well as statistical margin of error on F1-score at both 95% and 99% confidence interval.

## Test result
1. Mean F1 = 0.838
1. Var F1 = 0.000001627
1. Best F1 = 0.842
1. Worst F1 = 0.834
1. Margin of error at 95% = 0.000262
1. Margin of error at 99% = 0.004456
1. Mean tokenizer instantiation time = 94.21 ms
1. Var tokenizer instantiation time = 194.733 ms<sup>2</sup>
1. Margin of error at 95% for instantiation time = 2.783 ms
1. Mean tokenizer tokenization time = 261.55 ms
1. Var tokenizer tokenization time = 805.5152578072821 ms<sup>2</sup>
1. Margin of error at 95% for tokenization time = 5.659 ms
1. Mean total time = 261.74 ms
1. Var total time = 804.0327272727259 ms<sup>2</sup>
1. Margin of error at 95% for tokenization time = 5.654 ms

## Test result interpretation
For 100 random test with 42,499 concatenated words with 10% of it is unknown word by tokenizer. The 99% confidence F1-score fall between 0.8376-0.8384. 95% of time, it took between 256-267 ms to tokenize text. 95% of time, it took between 92-97 ms to load dictionary from memory. It took less than 1 ms to verify tokenization result.