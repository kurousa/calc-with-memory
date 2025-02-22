//! メモリ機能付き計算機
//!
//! 仕様
//!
//! - 計算式を1行ずつ読み込んで処理
//!   - 空白区切りで、数値 演算子 数値といった入力をすることで計算結果を出力する
//!   - mem+、mem-とだけ入力すると、直前の計算結果がメモリに足し引きされる
//!   - 計算式の 数値 の部分が、 mem となっていた場合、数値代わりにメモリの値を利用する
//!   - メモリは10個まで保持可能
//!   - メモリには名前付きで(例("名前",値)のタプルでの表現)が可能
//! - 計算結果は整数型ではなく小数型(f64)で管理
//!
//! 例
//! ```rust
//! 1 + 2
//! => 3
//! 3 * 4
//! => 12
//! mem+ 5
//! => 17
//! mem- 2
//! => 15
//! ```
use std::collections::{hash_map::Entry, HashMap};
use std::io::stdin;

/// メモリ構造体
struct Memory {
    /// メモリの名前と値のタプルを配列で保持
    slots: HashMap<String, f64>,
}

impl Memory {
    /// Memory構造体の初期化
    fn new() -> Self {
        Self {
            slots: HashMap::new(),
        }
    }
    /// メモリの追加、更新処理
    fn add(&mut self, slot_name: String, prev_result: f64) -> f64 {
        match self.slots.entry(slot_name) {
            Entry::Occupied(mut entry) => {
                *entry.get_mut() += prev_result;
                *entry.get()
            }
            Entry::Vacant(entry) => {
                entry.insert(prev_result);
                prev_result
            }
        }
    }
    /// メモリの値取得処理
    fn get(&self, slot_name: &str) -> f64 {
        self.slots.get(slot_name).copied().unwrap_or(0.0)
    }
}

/// トークン列挙体
#[derive(Debug, PartialEq)]
enum Token {
    // 数値部
    Number(f64),
    // メモリ参照
    MemoryRef(String),
    // 加算メモリ
    MemoryPlus(String),
    // 減算メモリ
    MemoryMinus(String),
    // 加算演算子
    Plus,
    // 減算演算子
    Minus,
    // 乗算演算子
    Asterisk,
    // 除算演算子
    Slash,
}
impl Token {
    /// トークンのパース処理
    fn parse(value: &str) -> Self {
        match value {
            "+" => Self::Plus,
            "-" => Self::Minus,
            "*" => Self::Asterisk,
            "/" => Self::Slash,
            // 上記にあてはまらないかつ、memで始まる場合
            _ if value.starts_with("mem") => {
                let mut memory_name = value[3..].to_string();
                if value.ends_with('+') {
                    memory_name.pop();
                    Self::MemoryPlus(memory_name)
                } else if value.ends_with('-') {
                    memory_name.pop();
                    Self::MemoryMinus(memory_name)
                } else {
                    Self::MemoryRef(memory_name)
                }
            }
            _ => Self::Number(value.parse().unwrap()),
        }
    }
    /// 入力値の分割とパース処理結果の取得
    fn split(text: &str) -> Vec<Self> {
        text.split(char::is_whitespace).map(Self::parse).collect()
    }
}
/// メイン処理
fn main() {
    let mut memories: Memory = Memory::new();
    let mut prev_result: f64 = 0.0;
    println!("Please input calculation formula like 1 + 2, 2 - 1, 3 * 4, 4 / 2");
    for line in stdin().lines() {
        let line = line.unwrap();
        if line.is_empty() {
            // 空行の場合処理を終了
            print!("Bye!");
            break;
        }
        // 入力を空白区切りで分割
        let tokens: Vec<Token> = Token::split(&line);

        // トークンによって処理を分岐
        match &tokens[0] {
            Token::MemoryPlus(memory_name) => {
                let memory_name = memory_name.to_string();
                let result = memories.add(memory_name, prev_result);
                print_formula_result(line, result);
            }
            Token::MemoryMinus(memory_name) => {
                let memory_name = memory_name.to_string();
                let result = memories.add(memory_name, -prev_result);
                print_formula_result(line, result);
            }
            _ => {
                let result = eval_expression(&tokens, &memories);
                print_formula_result(line, result);
                prev_result = result;
            }
        }
    }
}

/// トークンの解釈処理
fn eval_token(token: &Token, memory: &Memory) -> f64 {
    match token {
        Token::Number(value) => *value,
        Token::MemoryRef(memory_name) => memory.get(memory_name),
        _ => unreachable!(),
    }
}
/// 式の計算処理の解釈
fn eval_expression(tokens: &[Token], memory: &Memory) -> f64 {
    eval_additive_expression(tokens, memory)
}
/// 加減算処理
fn eval_additive_expression(tokens: &[Token], memory: &Memory) -> f64 {
    let mut index = 0;
    let mut result;

    (result, index) = eval_multiplicative_expression(tokens, index, memory);
    while index < tokens.len() {
        match &tokens[index] {
            Token::Plus => {
                let (value, next) = eval_multiplicative_expression(tokens, index + 1, memory);
                result += value;
                index = next;
            }
            Token::Minus => {
                let (value, next) = eval_multiplicative_expression(tokens, index + 1, memory);
                result -= value;
                index = next;
            }
            _ => break,
        }
    }
    result
}
/// 乗除算処理
fn eval_multiplicative_expression(tokens: &[Token], index: usize, memory: &Memory) -> (f64, usize) {
    let mut index: usize = index;
    let mut result: f64 = eval_token(&tokens[index], memory);
    index += 1;

    while index < tokens.len() {
        match &tokens[index] {
            Token::Asterisk => {
                result *= eval_token(&tokens[index + 1], memory);
                index += 2
            }
            Token::Slash => {
                result /= eval_token(&tokens[index + 1], memory);
                index += 2
            }
            _ => break,
        }
    }
    (result, index)
}
/// 計算結果出力
fn print_formula_result(formula: String, result: f64) {
    println!("{} equal {}", formula, result);
}
