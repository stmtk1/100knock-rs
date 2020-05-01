use cabocha::parser::Parser;
use regex::Regex;
use itertools::Itertools;

fn main() {
    //let res_txt = Parser::new(String::from("-f1")).parse_to_str(String::from("しかしその当時は考えもなかったから別段私は恐ろしいとも思わなかった。"));
    //let res_txt = Parser::new(String::from("-f1")).parse_to_str(String::from("吾輩はここで始めて人間というものを見た。"));
    //let res_txt = Parser::new(String::from("-f1")).parse_to_str(String::from("別段くるにも及ばんさと、主人は手紙に返事をする。"));
    let res_txt = Parser::new(String::from("-f1")).parse_to_str(String::from("吾輩はここで始めて人間というものを見た。"));
    let chunks: Vec<Chunk> = Chunk::new(res_txt);
    let verb: &Chunk = (&chunks).into_iter().rev().find(|w| w.include_pos(&String::from("動詞"))).unwrap();
    let noun: &Chunk = (&chunks).into_iter().rev().find(|w| match w.parent { None => false, Some(pid) => pid == verb.id }).unwrap();

    for chunk in &chunks {
        if let Some(pid) = chunk.parent {
            chunk.print_path(&chunks);
        }
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
                    let word = Word::new(&w).unwrap();
                    if word.pos != "記号" {
                        chunk.words.push(word);
                    }
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

     fn find_last_pos(&self, pos: String) -> Option<Word> {
         (&self.words).into_iter().rev().find(|w| w.pos == pos).and_then(|w| Some(w.clone()))
     }

     fn print_path(&self, chunks: &Vec<Chunk>) {
         match self.parent {
             None => println!("{}", self.join_words()),
             Some(pid) => {
                 print!("{} -> ", self.join_words());
                 chunks[pid].print_path(&chunks);
             }
         }
     }
}
