use std::collections::HashMap;


const KATAKANA: &'static [[&'static str; 5]; 32] = &[
    // everything except "n"
    //     a       i       u       e       o
    [   "ア",   "イ",   "ウ",   "エ",   "オ"],     //
    [   "カ",   "キ",   "ク",   "ケ",   "コ"],     // k
    [   "サ",   "シ",   "ス",   "セ",   "ソ"],     // s
    [   "タ",   "チ",   "ツ",   "テ",   "ト"],     // t
    [   "ナ",   "ニ",   "ヌ",   "ネ",   "ノ"],     // n
    [   "ハ",   "ヒ",   "フ",   "ヘ",   "ホ"],     // h
    [   "マ",   "ミ",   "ム",   "メ",   "モ"],     // m
    [   "ヤ",     "",   "ユ", "イェ",   "ヨ"],     // y
    [   "ラ",   "リ",   "ル",   "レ",   "ロ"],     // r
    [   "ワ", "ウィ",     "", "ウェ",   "ヲ"],     // w

    [   "ガ",   "ギ",   "グ",   "ゲ",   "ゴ"],     // g
    [   "ザ",   "ジ",   "ズ",   "ゼ",   "ゾ"],     // z
    [   "ダ",   "ヂ",   "ヅ",   "デ",   "ド"],     // d
    [   "バ",   "ビ",   "ブ",   "ベ",   "ボ"],     // b
    [   "パ",   "ピ",   "プ",   "ペ",   "ポ"],     // p

    [ "キャ", "キィ", "キュ", "キェ", "キョ"],     // ky
    [ "シャ",   "シ", "シュ", "シェ", "ショ"],     // sh
    [ "チャ",   "チ", "チュ", "チェ", "チョ"],     // ch
    [ "ニャ", "ニィ", "ニュ", "ニェ", "ニョ"],     // ny
    [ "ヒャ", "ヒィ", "ヒュ", "ヒェ", "ヒョ"],     // hy
    [ "ミャ", "ミィ", "ミュ", "ミェ", "ミョ"],     // my
    [ "リャ", "リィ", "リュ", "リェ", "リョ"],     // ry
    [ "ギャ", "ギィ", "ギュ", "ギェ", "ギョ"],     // gy
    [ "ジャ",   "ジ", "ジュ", "ジェ", "ジョ"],     // j
    [ "ビャ", "ビィ", "ビュ", "ビェ", "ビョ"],     // by
    [ "ピャ", "ピィ", "ピュ", "ピェ", "ピョ"],     // py

    [ "ツァ", "ツィ",   "ツ", "ツェ", "ツォ"],     // ts
    [ "ファ", "フィ",   "フ", "フェ", "フォ"],     // f
    [ "ヴァ", "ヴィ",   "ヴ", "ヴェ", "ヴォ"],     // v
    [   "ァ",   "ィ",   "ゥ",   "ェ",   "ォ"],     // x, l
    [   "カ",   "シ",   "ク",   "セ",   "コ"],     // c
    [ "クァ", "クィ",   "ク", "クェ", "クォ"],     // q

    // "n", will handle directly in public function
    // [ "ン"],
];


lazy_static! {
    static ref KATAKANA_ROW_INDEX: HashMap<&'static str, usize> = {
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

    static ref KATAKANA_COLUMN_INDEX: HashMap<char, usize> = {
        let mut m = HashMap::new();
        m.insert('a', 0);
        m.insert('i', 1);
        m.insert('u', 2);
        m.insert('e', 3);
        m.insert('o', 4);
        m
    };
}

pub fn romaji_to_katakana(data: &str) -> String {
    let mut result = String::new();
    let mut keep = String::new();
    let mut it = data.chars();

    while let Some(c) = it.next() {
        match KATAKANA_COLUMN_INDEX.get(&c) {
            Some(col) => {
                // too many things
                // it has "n"s or something we can't translate
                if keep.len() > 2 {
                    {
                        let part1 = &keep[..keep.len()-2];
                        for i in part1.chars() {
                            result.push(if i == 'n' { 'ン' } else { i });
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
                    result.push(if c1 == 'n' { 'ン' } else { c1 });
                    keep.clear();
                    keep.push(c2);
                }

                // lookup the table
                match KATAKANA_ROW_INDEX.get(keep.as_str()) {
                    Some(row) => { result.push_str(KATAKANA[*row][*col]) },
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
        result.push(if c == 'n' { 'ン' } else { c });
    }

    result
}
