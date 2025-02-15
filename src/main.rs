/// メモリ機能付き電卓
///
/// 仕様
///
/// - 計算式を1行ずつ読み込んで処理
///   - 空白区切りで、数値 演算子 数値といった入力をすることで計算結果を出力する
///   - mem+、mem-とだけ入力すると、直前の計算結果がメモリに足し引きされる
///   - 計算式の 数値 の部分が、 mem となっていた場合、数値代わりにメモリの値を利用する
///   - メモリは10個まで保持可能
/// - 計算結果は整数型ではなく小数型(f64)で管理
///
/// 例
/// ```rust
/// 1 + 2
/// => 3
/// ``
use std::io::stdin;
fn main() {
    let mut memories: Vec<f64> = vec![0.0; 10];
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
            set_memory_print_value(&mut memories, first_token, prev_result);
            continue;
        } else if is_memory && first_token.ends_with('-') {
            set_memory_print_value(&mut memories, first_token, -prev_result); // memoryの参照渡し
            continue;
        }

        let third_token = tokens[2];
        // 数値部を取得
        // memの場合はmemoryの値を利用
        let left: f64 = eval_token(first_token, &memories);
        let right: f64 = eval_token(third_token, &memories);
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

fn eval_token(token: &str, memories: &[f64]) -> f64 {
    if token.starts_with("mem") {
        let slot_index: usize = token[3..].parse().unwrap();
        memories[slot_index]
    } else {
        token.parse().unwrap()
    }
}

fn set_memory_print_value(memories: &mut [f64], token: &str, prev_result: f64) {
    // tokenの3文字目から最後の文字の1つ前までを取得
    // 例: mem10+ といった文字列の場合、10を取得
    let slot_index: usize = token[3..token.len() - 1].parse().unwrap();
    memories[slot_index] += prev_result;
    print_formula_result(format!("set memory{}", slot_index), memories[slot_index]);
}
