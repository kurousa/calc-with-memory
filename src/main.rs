use std::collections::{hash_map::Entry, HashMap};
///
/// 仕様
///
/// - 計算式を1行ずつ読み込んで処理
///   - 空白区切りで、数値 演算子 数値といった入力をすることで計算結果を出力する
///   - mem+、mem-とだけ入力すると、直前の計算結果がメモリに足し引きされる
///   - 計算式の 数値 の部分が、 mem となっていた場合、数値代わりにメモリの値を利用する
///   - メモリは10個まで保持可能
///   - メモリには名前付きで(例("名前",値)のタプルでの表現)が可能
/// - 計算結果は整数型ではなく小数型(f64)で管理
///
/// 例
/// ```rust
/// 1 + 2
/// => 3
/// ``
use std::io::stdin;
/// メモリ構造体
struct Memory {
    /// メモリの名前と値のタプルを配列で保持
    slots: HashMap<String, f64>,
}
/// Memory構造体に関わるメソッドの実装
impl Memory {
    /// Memory構造体の初期化
    fn new() -> Self {
        Self {
            slots: HashMap::new(),
        }
    }
    /// tokenがmemで始まる場合はメモリの値を返却
    fn eval_token(&self, token: &str) -> f64 {
        if token.starts_with("mem") {
            let slot_name: &str = &token[3..];
            // self.slots.get(slot_name)の戻り値は、Option<&f64>で中身は参照
            // そのままでは値を返せないため、copied()でOption<f64>に変換
            // unwrap_or(0.0)でOption<f64>の中身がNoneの場合は0.0を返却
            self.slots.get(slot_name).copied().unwrap_or(0.0)
        } else {
            token.parse().unwrap()
        }
    }
    /// メモリに値を設定し、コンソールに出力
    fn set_memory_print_value(&mut self, token: &str, prev_result: f64) {
        // tokenの3文字目から最後の文字の1つ前までを取得
        // 例: mem10+ といった文字列の場合、10を取得
        let slot_name = &token[3..token.len() - 1];
        match self.slots.entry(slot_name.to_string()) {
            Entry::Occupied(mut entry) => {
                // メモリが見つかった場合は値を更新
                *entry.get_mut() += prev_result;
                print_formula_result(format!("set memory{}", slot_name), *entry.get());
            }
            Entry::Vacant(entry) => {
                // メモリが見つからなかった場合は末尾に追加
                entry.insert(prev_result);
                print_formula_result(format!("set memory{}", slot_name), prev_result);
            }
        }
    }
}

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
        let tokens: Vec<&str> = line.split(char::is_whitespace).collect();
        let first_token = tokens[0];

        // メモリ機能の処理
        let is_memory = first_token.starts_with("mem");
        if is_memory && first_token.ends_with('+') {
            memories.set_memory_print_value(first_token, prev_result);
            continue;
        } else if is_memory && first_token.ends_with('-') {
            memories.set_memory_print_value(first_token, -prev_result); // memoryの参照渡し
            continue;
        }

        let third_token = tokens[2];
        // 数値部を取得
        // memの場合はmemoryの値を利用
        let left: f64 = memories.eval_token(first_token);
        let right: f64 = memories.eval_token(third_token);
        let expression: &str = tokens[1];
        let result: f64 = match expression {
            "+" => add_value(left, right),
            "-" => sub_value(left, right),
            "*" => multiply_value(left, right),
            "/" => divide_value(left, right),
            _ => unreachable!("Invalid operator, use only +, -, *, /"),
        };
        print_formula_result(line, result);
        prev_result = result;
    }
}

// 計算結果出力
fn print_formula_result(formula: String, result: f64) {
    println!("{} equal {}", formula, result);
}

// 加算処理
fn add_value(left: f64, right: f64) -> f64 {
    left + right
}
// 減算処理
fn sub_value(left: f64, right: f64) -> f64 {
    left - right
}
// 乗算処理
fn multiply_value(left: f64, right: f64) -> f64 {
    left * right
}
// 除算処理
fn divide_value(left: f64, right: f64) -> f64 {
    left / right
}
