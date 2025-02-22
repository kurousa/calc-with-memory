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
                let left = eval_token(&tokens[0], &memories);
                let right = eval_token(&tokens[2], &memories);
                let result = eval_expression(left, &tokens[1], right);
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
fn eval_expression(left: f64, operator: &Token, right: f64) -> f64 {
    match operator {
        Token::Plus => add_value(left, right),
        Token::Minus => sub_value(left, right),
        Token::Asterisk => multiply_value(left, right),
        Token::Slash => divide_value(left, right),
        _ => unreachable!("Invalid operator, use only +, -, *, /"),
    }
}
/// 計算結果出力
fn print_formula_result(formula: String, result: f64) {
    println!("{} equal {}", formula, result);
}

/// 加算処理
fn add_value(left: f64, right: f64) -> f64 {
    left + right
}
/// 減算処理
fn sub_value(left: f64, right: f64) -> f64 {
    left - right
}
/// 乗算処理
fn multiply_value(left: f64, right: f64) -> f64 {
    left * right
}
/// 除算処理
fn divide_value(left: f64, right: f64) -> f64 {
    left / right
}
