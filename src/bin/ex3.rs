fn main() {
    let abastecimento_litros = teste_modelo_0782::read_input_and_parse::<f64>("Abastecimento (L): ");
    let preço_por_litro = teste_modelo_0782::read_input_and_parse::<f64>("Preço por litro (EUR): ");
    let total = abastecimento_litros * preço_por_litro;
    let iva = total * 0.23;
    println!("****** FATURA ******");
    println!("Total: {:.2} EUR", total);
    println!("IVA: {:.2} EUR", iva);
}
