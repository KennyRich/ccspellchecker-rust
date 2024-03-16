# Building a Bloom Filter Spell Checker

This project was inspired by this [Article](https://codingchallenges.fyi/challenges/challenge-bloom)

To build a Bloom Filter from a dictionary file, use the `--build` option followed by the path to the dictionary file:

```shell
./ccspellcheck --build /path/to/dictionary.txt
```

This will create a Bloom Filter file named `default_words.bf` in the current directory.

### Checking Words

To check if words are present in the dictionary, use the `--check` option followed by one or more words:

```shell
./ccspellcheck --check hello world programming
```

The program will output whether each word is found in the dictionary or not.

### Customizing Bloom Filter Parameters

You can customize the size of the Bloom Filter and the number of hash functions used by providing the `--size` and `--num_hashes` options, respectively:

```shell
./ccspellcheck --build /path/to/dictionary.txt --size 1000000 --num_hashes 5
```
