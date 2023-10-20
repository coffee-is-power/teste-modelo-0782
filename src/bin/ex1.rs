fn main() {
    let width = teste_modelo_0782::read_input_and_parse::<i64>("Introduza a largura do retangulo: ");
    let height = teste_modelo_0782::read_input_and_parse::<i64>("Introduza a altura do retangulo: ");
    let area = width * height;
    println!("A área do retangulo é: {area}");
}
