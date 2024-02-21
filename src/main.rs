fn main() {
    // Ler os operandos
    println!("Digite o primeiro número:");
    let mut num1 = String::new();
    std::io::stdin().read_line(&mut num1).unwrap();
    let num1: f64 = num1.trim().parse().unwrap();

    println!("Digite o segundo número:");
    let mut num2 = String::new();
    std::io::stdin().read_line(&mut num2).unwrap();
    let num2: f64 = num2.trim().parse().unwrap();

    // Escolha a operação
    println!("Digite a operação (+, -, *, /):");
    let mut op = String::new();
    std::io::stdin().read_line(&mut op).unwrap();
    let op = op.trim();

    // Calcular o resultado
    let result = match op {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _ => panic!("Operação inválida!"),
    };

    // Exibir o resultado
    println!("Resultado: {}", result);
}
