/// 電卓
///
/// 仕様
///
/// - 計算式を1行ずつ読み込んで処理
///   - 空白区切りで、数値 演算子 数値といった入力をすることで計算結果を出力する
/// - 計算結果は整数型ではなく小数型(f64)で管理
///
/// 例
/// ```rust
/// 1 + 2
/// => 3
/// ``
use std::io::stdin;
fn main() {
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
        // 数値と演算子(+-*/)を取得
        // 所定の演算子出ない場合は異常終了する
        let left: f64 = tokens[0].parse().unwrap();
        let right: f64 = tokens[2].parse().unwrap();
        let result: f64 = match tokens[1] {
            "+" => add_value(left, right),
            "-" => sub_value(left, right),
            "*" => multiply_value(left, right),
            "/" => divide_value(left, right),
            _ => unreachable!("Invalid operator, use only +, -, *, /"),
        };
        print_formula_result(line, result);
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
