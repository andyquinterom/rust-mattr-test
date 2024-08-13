use std::io::{read_to_string, Read};

mod original {
    use std::collections::HashSet;

    fn get_ttr(inlist: &[&str]) -> f64 {
        let n_tokens = &inlist.len();
        let n_types = &inlist.into_iter().collect::<HashSet<_>>().len();
        return *n_types as f64 / *n_tokens as f64;
    }

    pub fn get_mattr(in_vector: &[&str], window_span: usize) -> f64 {
        let n_wds = in_vector.len();
        let output: f64;
        if n_wds <= 50 {
            output = get_ttr(in_vector);
        } else {
            let mut numerator = 0.0;
            let n_window = n_wds - window_span + 1;
            for cur_window in in_vector.windows(window_span) {
                numerator += get_ttr(cur_window)
            }
            output = numerator / n_window as f64;
        }
        output
    }
}

mod my_version {
    // the hashbrown hashset is faster because is uses
    // fxhash
    use hashbrown::HashSet;

    // we take in usizes instead of strings and use a pre-allocated
    // hashset to make things go faster
    fn get_ttr(inlist: &[usize], set: &mut HashSet<usize>) -> f64 {
        // we clear the set and add the values into it
        set.clear();
        set.extend(inlist);
        let n_tokens = inlist.len();
        let n_types = set.len();
        n_types as f64 / n_tokens as f64
    }

    pub fn get_mattr(in_vector: &[&str], window_span: usize) -> f64 {
        let n_wds = in_vector.len();
        // we replace the previous in_vector with a new one
        // that has the hashes of the strings
        // instead of the strings
        let in_vector: Vec<_> = in_vector.iter().map(fxhash::hash).collect();
        let mut set = HashSet::with_capacity(window_span);
        if n_wds <= 50 {
            get_ttr(&in_vector, &mut set)
        } else {
            let n_window = n_wds - window_span + 1;
            let numerator: f64 = in_vector
                .windows(window_span)
                .map(|window| get_ttr(window, &mut set))
                .sum();
            numerator / n_window as f64
        }
    }
}

fn main() {
    let dream = std::fs::File::open("dream.txt").expect("Unable to read file");
    let text = read_to_string(dream)
        .expect("failed to read text")
        .to_uppercase();
    let words: Vec<&str> = text.split_whitespace().collect();

    println!("{}", words.len());

    const TIMES: usize = 5;
    const WINDOW: usize = 1;

    for _ in 0..TIMES {
        let start = std::time::Instant::now();
        let result = my_version::get_mattr(&words, WINDOW);
        let elapsed = start.elapsed();
        println!("my_version,{result},{:?}", elapsed);
    }
    for _ in 0..TIMES {
        let start = std::time::Instant::now();
        let result = original::get_mattr(&words, WINDOW);
        let elapsed = start.elapsed();
        println!("original,{result},{:?}", elapsed);
    }
}
