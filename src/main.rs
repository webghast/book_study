use std::io;
use 
fn main() {
    let mut userinput = String::new();
    let mut userinput2 = String::new();

    println!("Por favor me diga seu nome");

    io::stdin()
        .read_line(&mut userinput)
        .expect("Falha ao anotar seu nome!");

    println!("Agora me diga sua idade!");

    io::stdin()
        .read_line(&mut userinput2)
        .expect("Falaha ao anotar sua idade!");

    println!("Pelos meus registros seu nome consta como : {userinput}");
    println!("e sua idade como {userinput2}");
}
