fn main() {
    println!("请输入第一个数字：");
    let mut num1 = String::new();
    std::io::stdin().read_line(&mut num1).expect("读取失败");
    let num1: i32 = num1.trim().parse().expect("输入的不是数字");
    println!("请输入第二个数字：");
    let mut num2 = String::new();
    std::io::stdin().read_line(&mut num2).expect("读取失败");
    let num2: i32 = num2.trim().parse().expect("输入的不是数字");
    println!("请输入运算符：");
    let mut operator = String::new();
    std::io::stdin().read_line(&mut operator).expect("读取失败");
    let operator = operator.trim();
    let operator = match operator {
        "+" => Operator::Add(operator.to_string()),
        "-" => Operator::Sub(operator.to_string()),
        "*" => Operator::Mul(operator.to_string()),
        "/" => Operator::Div(operator.to_string()),
        _ => panic!("不支持的运算符"),
    };
    let result = match operator {
        Operator::Add(_) => num1 + num2,
        Operator::Sub(_) => num1 - num2,
        Operator::Mul(_) => num1 * num2,
        Operator::Div(_) => num1 / num2,
    };
    println!("{} {} {} = {}", num1, operator_to_string(&operator), num2, result);
}

enum Operator {
    Add(String),
    Sub(String),
    Mul(String),
    Div(String),
}

impl Operator {
    fn to_string(&self) -> String {
        match self {
            Operator::Add(s) => s.to_string(),
            Operator::Sub(s) => s.to_string(),
            Operator::Mul(s) => s.to_string(),
            Operator::Div(s) => s.to_string(),
        }
    }
}

fn operator_to_string(operator: &Operator) -> String {
    operator.to_string()
}
