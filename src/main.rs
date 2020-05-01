use cabocha::parser::Parser;
use regex::Regex;
use itertools::Itertools;

fn main() {
    //let res_txt = Parser::new(String::from("-f1")).parse_to_str(String::from("しかしその当時は考えもなかったから別段私は恐ろしいとも思わなかった。"));
    let res_txt = Parser::new(String::from("-f1")).parse_to_str(String::from("吾輩はここで始めて人間というものを見た。"));
    let chunks: Vec<Chunk> = Chunk::new(res_txt);

    for chunk in &chunks {
        if chunk.words[0].pos != "動詞" {
            continue;
        }
        print!("{} ", chunk.words[0].base);
        for other in &chunks {
            let last = other.words.last().unwrap();
            if other.parent.is_some() && other.parent.unwrap() == chunk.id && last.pos == "助詞" {
                print!("{} ", last.surface);
            }
        }
        println!("");
    }
}

#[derive(Debug, Clone)]
enum Morph {
    Word {
        surface: String,
        pos: String,
        base: String,
        pos1: String,
    },
    Section{
        id: usize,
        parent: Option<usize>,
    },
}

#[derive(Debug, Clone)]
struct Word {
    surface: String,
    pos: String,
    base: String,
    pos1: String,
}

impl Morph {
    fn new(s :String) -> Option<Morph> {
        let parts: Vec<String> = Regex::new(r"[\s,]").unwrap().split(&format!("{}", s)).map(|a| String::from(a)).collect();
        if parts.len() <= 1 {
            None
        } else if parts[0] == String::from("*")  {
            let parent_id: i64 = parts[2].replace("D", "").parse().unwrap();
            let parent = if parent_id < 0 {
                None
            } else {
                Some(parent_id as usize)
            };
            Some(Morph::Section{ 
                id: parts[1].parse().unwrap(),
                parent
            })
        } else {
            Some(Morph::Word {
                surface: parts[0].clone(),
                pos: parts[1].clone(),
                pos1: parts[2].clone(),
                base: parts[7].clone(),
            })
        }
    }

    fn id(&self) -> Option<usize> {
        match self {
            Morph::Section{ id, .. } => Some(*id),
            Morph::Word{..} => None,
        }
    }

    fn parent(&self) -> Option<Option<usize>> {
        match self {
            Morph::Section{ parent, .. } => Some(*parent),
            Morph::Word{..} => None,
        }
    }
}

impl Word {
    fn new(morph: &Morph) -> Option<Word> {
        match morph {
            Morph::Section{..} => None,
            Morph::Word { surface, pos, pos1, base } => {
                Some(Word{
                    surface: surface.clone(),
                    pos: pos.clone(),
                    pos1: pos1.clone(),
                    base: base.clone(),
                })
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Chunk {
    id: usize,
    parent: Option<usize>,
    words: Vec<Word>,
}

impl Chunk {
    fn new(s: String) -> Vec<Chunk> {
        let mut ret: Vec<Chunk> = Vec::new();
        let words: Vec<Morph> = s.split("\n").into_iter().map(|s| Morph::new(String::from(s))).filter(|w| w.is_some()).map(|w| w.unwrap()).collect();
        let first_id = words[0].id().unwrap();
        let first_parent = words[0].parent().unwrap();
        let mut chunk = Chunk { id: first_id, words: Vec::new(), parent: first_parent };
        for w in words.into_iter().skip(1) {
            match w {
                Morph::Section{ id, parent } => {
                    ret.push(chunk);
                    chunk = Chunk {
                        id,
                        parent,
                        words: Vec::new(),
                    };
                },
                Morph::Word{..} => {
                    chunk.words.push(Word::new(&w).unwrap());
                }
            }
        }
        ret.push(chunk);
        ret
    }

    fn print_words(&self) {
        for word in &self.words {
            let Word { surface, .. } = word;
            print!("{}", surface);
        }
    }

    fn join_words(&self) -> String {
        (&self.words).into_iter().map(|w| &w.surface as &str).intersperse("").collect()
    }

     fn include_pos(&self, pos: &String) -> bool {
         (&self.words).into_iter().find(|w| &w.pos == pos ).is_some()
     }
}
