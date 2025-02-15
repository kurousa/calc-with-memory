/// メモリ機能付き電卓
///
/// 仕様
///
/// - 計算式を1行ずつ読み込んで処理
///   - 空白区切りで、数値 演算子 数値といった入力をすることで計算結果を出力する
///   - mem+、mem-とだけ入力すると、直前の計算結果がメモリに足し引きされる
///   - 計算式の 数値 の部分が、 mem となっていた場合、数値代わりにメモリの値を利用する
/// - 計算結果は整数型ではなく小数型(f64)で管理
///
/// 例
/// ```rust
/// 1 + 2
/// => 3
/// ``
use std::io::stdin;
fn main() {
    let mut memory: f64 = 0.0;
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
        if first_token == "mem+" {
            set_memory_print_value(&mut memory, prev_result); // memoryの参照渡し
            continue;
        } else if first_token == "mem-" {
            set_memory_print_value(&mut memory, -prev_result); // memoryの参照渡し
            continue;
        }

        let third_token = tokens[2];
        // 数値部を取得
        // memの場合はmemoryの値を利用
        let left: f64 = eval_token(first_token, memory);
        let right: f64 = eval_token(third_token, memory);
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

fn eval_token(token: &str, memory: f64) -> f64 {
    if token == "mem" {
        memory
    } else {
        token.parse().unwrap()
    }
}

fn set_memory_print_value(memory: &mut f64, prev_result: f64) {
    *memory += prev_result;
    print_formula_result("set memory".to_string(), *memory);
}
