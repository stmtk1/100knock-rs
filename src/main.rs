use std::fs::{ self, File, OpenOptions };
use rand::seq::SliceRandom;
use std::io::{ Write, BufWriter };

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let train_news: Vec<News> = fs::read_to_string("train.txt")?.split('\n').filter_map(|s| News::new(String::from(s))).collect();
    let valid_news: Vec<News> = fs::read_to_string("valid.txt")?.split('\n').filter_map(|s| News::new(String::from(s))).collect();
    let test_news: Vec<News> = fs::read_to_string("test.txt")?.split('\n').filter_map(|s| News::new(String::from(s))).collect();

    let mut train = BufWriter::new(OpenOptions::new().create(true).append(true).open("train.feature.txt")?);
    let mut valid = BufWriter::new(OpenOptions::new().create(true).append(true).open("valid.feature.txt")?);
    let mut test = BufWriter::new(OpenOptions::new().create(true).append(true).open("test.feature.txt")?);

    for n in &train_news {
        train.write(format!("{}\t{}\t{}\t{}\t{}\t{}\n", n.category, n.word_num, n.start_quote, n.start_capital, n.wl_average, n.wl_max).as_bytes())?;
    }

    for n in &valid_news {
        valid.write(format!("{}\t{}\t{}\t{}\t{}\t{}\n", n.category, n.word_num, n.start_quote, n.start_capital, n.wl_average, n.wl_max).as_bytes())?;
    }

    for n in &test_news {
        test.write(format!("{}\t{}\t{}\t{}\t{}\t{}\n", n.category, n.word_num, n.start_quote, n.start_capital, n.wl_average, n.wl_max).as_bytes())?;
    }

    train.flush()?;
    test.flush()?;
    valid.flush()?;
    Ok(())
}

struct News {
    category: String,
    word_num: usize,
    start_quote: bool,
    start_capital: bool,
    wl_average: usize,
    wl_max: usize,
}

impl News {
    fn new(s: String) -> Option<News> {
        let v: Vec<String> = s.split('\t').map(|a| String::from(a)).collect();
        if v.len() != 2 {
            return None;
        }
        let a = v[1].split(' ').collect::<Vec<&str>>();
        let mut wl_average = 0;
        let mut wl_max = 0;
        let start_char = v[1].as_bytes()[0] as char;

        for c in &a {
            let length = c.as_bytes().len();

            wl_average += length;
            wl_max = std::cmp::max(length, wl_max);
        }
        wl_average /= a.len();

        Some(News {
            category: v[0].clone(),
            word_num: a.len(),
            wl_max,
            wl_average,
            start_quote: start_char == '\'' || start_char == '"',
            start_capital: start_char.is_uppercase(),
        })
    }
}
