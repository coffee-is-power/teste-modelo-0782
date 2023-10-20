fn main() {
    let anos = [2022, 2023, 2024];
    let divisao = anos.iter().map(|ano| ano / 8).collect::<Vec<_>>();
    let resto = anos.iter().map(|ano| ano % 8).collect::<Vec<_>>();
    println!("Ano\t| Div\t| Resto\t| Bissexto");
    println!("------------------------------------");
    for ((ano, divisao), resto) in anos.into_iter().zip(divisao).zip(resto) {
        println!("{ano}\t| {divisao}\t| {resto}\t| {}", if resto == 0 { "Sim" } else { "Não" });
    }
}
