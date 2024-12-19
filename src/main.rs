

fn muitas_saidas(opção: &str) -> usize{
    if opção == "um" {
        1
    }
    else if opção == "dois" {
        2
    }
    else {
        0
    }
}


fn main() {

    println!("opçao escolhida: {}", muitas_saidas("um"));



    }





}