use std::collections::HashMap;


const HIRAGANA: &'static [[&'static str; 5]; 32] = &[
    // everything except "n"
    //     a       i       u       e       o
    [   "あ",   "い",   "う",   "え",   "お"],     //
    [   "か",   "き",   "く",   "け",   "こ"],     // k
    [   "さ",   "し",   "す",   "せ",   "そ"],     // s
    [   "た",   "ち",   "つ",   "て",   "と"],     // t
    [   "な",   "に",   "ぬ",   "ね",   "の"],     // n
    [   "は",   "ひ",   "ふ",   "へ",   "ほ"],     // h
    [   "ま",   "み",   "む",   "め",   "も"],     // m
    [   "や",     "",   "ゆ", "いぇ",   "よ"],     // y
    [   "ら",   "り",   "る",   "れ",   "ろ"],     // r
    [   "わ", "うぃ",   "う", "うぇ",   "を"],     // w

    [   "が",   "ぎ",   "ぐ",   "げ",   "ご"],     // g
    [   "ざ",   "じ",   "ず",   "ぜ",   "ぞ"],     // z
    [   "だ",   "ぢ",   "づ",   "で",   "ど"],     // d
    [   "ば",   "び",   "ぶ",   "べ",   "ぼ"],     // b
    [   "ぱ",   "ぴ",   "ぷ",   "ぺ",   "ぽ"],     // p

    [ "きゃ", "きぃ", "きゅ", "きぇ", "きょ"],     // ky
    [ "しゃ",   "し", "しゅ", "しぇ", "しょ"],     // sh
    [ "ちゃ",   "ち", "ちゅ", "ちぇ", "ちょ"],     // ch
    [ "にゃ", "にぃ", "にゅ", "にゃ", "にょ"],     // ny
    [ "ひゃ", "ひぃ", "ひゅ", "ひぇ", "ひょ"],     // hy
    [ "みゃ", "みぃ", "みゅ", "みぇ", "みょ"],     // my
    [ "りゃ", "りぃ", "りゅ", "りぇ", "りょ"],     // ry
    [ "ぎゃ", "ぎぃ", "ぎゅ", "ぎぇ", "ぎょ"],     // gy
    [ "じゃ",   "じ", "じゅ", "じぇ", "じょ"],     // j
    [ "びゃ", "びぃ", "びゅ", "びぇ", "びょ"],     // by
    [ "ぴゃ", "ぴぃ", "ぴゅ", "ぴぇ", "ぴょ"],     // py

    [ "つぁ", "つぃ",   "つ", "つぇ", "つぉ"],     // ts
    [ "ふぁ", "ふぃ",   "ふ", "ふぇ", "ふぉ"],     // f
    [ "ゔぁ", "ゔぃ",   "ゔ", "ゔぇ", "ゔぉ"],     // v
    [   "ぁ",   "ぃ",   "ぅ",   "ぇ",   "ぉ"],     // x, l
    [   "か",   "し",   "く",   "せ",   "こ"],     // c
    [ "くぁ", "くぃ",   "く", "くぇ", "くぉ"],     // q

    // "n", will handle directly in public function
    // [ "ん"],
];


lazy_static! {
    static ref HIRAGANA_ROW_INDEX: HashMap<&'static str, usize> = {
        let mut m = HashMap::new();
        m.insert("", 0);
        m.insert("k", 1);
        m.insert("s", 2);
        m.insert("t", 3);
        m.insert("n", 4);
        m.insert("h", 5);
        m.insert("m", 6);
        m.insert("y", 7);
        m.insert("r", 8);
        m.insert("w", 9);
        m.insert("g", 10);
        m.insert("z", 11);
        m.insert("d", 12);
        m.insert("b", 13);
        m.insert("p", 14);
        m.insert("ky", 15);
        m.insert("sh", 16);
        m.insert("ch", 17);
        m.insert("ny", 18);
        m.insert("hy", 19);
        m.insert("my", 20);
        m.insert("ry", 21);
        m.insert("gy", 22);
        m.insert("j", 23);
        m.insert("by", 24);
        m.insert("py", 25);
        m.insert("ts", 26);
        m.insert("f", 27);
        m.insert("v", 28);
        m.insert("x", 29);
        m.insert("l", 29);
        m.insert("c", 30);
        m.insert("q", 31);
        m
    };

    static ref HIRAGANA_COLUMN_INDEX: HashMap<char, usize> = {
        let mut m = HashMap::new();
        m.insert('a', 0);
        m.insert('i', 1);
        m.insert('u', 2);
        m.insert('e', 3);
        m.insert('o', 4);
        m
    };
}

pub fn romaji_to_hiragana(data: &str) -> String {
    // need: "hanagana" -> [("h", "a"), ("n", "a"), ("g", "a"), ("n", "a")]
    let mut result = String::new();
    let mut keep = String::new();
    let mut it = data.chars();

    while let Some(c) = it.next() {
        match HIRAGANA_COLUMN_INDEX.get(&c) {
            Some(col) => {
                // too many things
                // it has "n"s or something we can't translate
                if keep.len() > 2 {
                    {
                        let part1 = &keep[..keep.len()-2];
                        for i in part1.chars() {
                            result.push(if i == 'n' { 'ん' } else { i });
                        }
                    }
                    let part2 = &keep[keep.len()-2..].to_string();
                    keep.clear();
                    keep.push_str(part2.as_str());
                }

                // it has "n" or something we can't translate
                if keep.len() == 2 &&
                    !["ky", "sh", "ch", "ny", "hy",
                      "my", "ry", "gy", "by", "py",
                      "ts"].contains(&keep.as_str()) {
                    let (c1, c2) = {
                        let mut it = keep.chars().take(2);
                        (it.next().unwrap(),
                         it.next().unwrap())
                    };
                    result.push(if c1 == 'n' { 'ん' } else { c1 });
                    keep.clear();
                    keep.push(c2);
                }

                // lookup the table
                match HIRAGANA_ROW_INDEX.get(keep.as_str()) {
                    Some(row) => { result.push_str(HIRAGANA[*row][*col]) },
                    None => {
                        result.push_str(&keep);
                        result.push(c);
                    },
                }
                keep.clear();
            },
            None => keep.push(c),
        }
    }

    // trailing "n"s and something we can't translate
    for c in keep.chars() {
        result.push(if c == 'n' { 'ん' } else { c });
    }

    result
}
