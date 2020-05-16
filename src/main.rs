use std::fs::{ self, File, OpenOptions };
use rand::seq::SliceRandom;
use std::io::{ Write, BufWriter };

const P1: &str = "Reuters";
const P2: &str = "Huffington Post";
const P3: &str = "Businessweek";
const P4: &str = "Contactmusic.com";
const P5: &str = "Daily Mail";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string("./newsCorpora.csv")?;
    let lines: Vec<Vec<String>> = content.split('\n').map(|s| s.split('\t').map(|a| String::from(a)).collect()).collect();
    let mut news: Vec<News> = lines.into_iter().filter(|v| v.len() >= 8 && (P1 == v[3] || P2 == v[3] || P3 == v[3] || P4 == v[3] || P5 == v[3])).map(|s| News::new(s)).collect();
    let mut train = BufWriter::new(OpenOptions::new().create(true).append(true).open("train.txt")?);
    let mut valid = BufWriter::new(OpenOptions::new().create(true).append(true).open("valid.txt")?);
    let mut test = BufWriter::new(OpenOptions::new().create(true).append(true).open("test.txt")?);

    let mut rng = rand::thread_rng();
    news.shuffle(&mut rng);

    for (i, n) in (&news).into_iter().enumerate() {
        if i < news.len() / 10 * 8 {
            train.write(format!("{}\t{}\n", n.category, n.title).as_bytes())?;
        } else if i < news.len() / 10 * 9 {
            valid.write(format!("{}\t{}\n", n.category, n.title).as_bytes())?;
        } else {
            test.write(format!("{}\t{}\n", n.category, n.title).as_bytes())?;
        }
    }

    train.flush()?;
    test.flush()?;
    valid.flush()?;
    Ok(())
}

struct News {
    category: String,
    title: String,
}

impl News {
    fn new(s: Vec<String>) -> News {
        News {
            category: s[4].clone(),
            title: s[1].clone(),
        }
    }
}
