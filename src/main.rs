

fn main() {
    // playing_with_numbers();
    // playing_with_chars_and_tuples();
    lets_test_references();
}


fn playing_with_numbers(){
    println!("Hello, world!");

    let number1: u16 = 10000;
    let number2: u32 = 15_0000;
    let mut result: u32;
    let mut result16: u16 = 0;

    result = number1 as u32 + number2;
    println!("The result is {}", result);

    result = number1 as u32 * 3 + number2;
    println!("The result is {}", result);

    // Let's play with overflow:

    // This would run forever in release and would panic on debug.
    // loop {
    //     result16 += 10;
    //
    // }
    //

    // This would cause an error by the compiler.
    // result16 = number1 * number1;
    // println!("The result is {}", result16);

    // This would panic on debug and release.
    // loop {
    //     result16 = result16.checked_add(1000).expect("That's an overflow!");
    //     println!("The result is {}", result16);
    // }

}


fn playing_with_chars_and_tuples(){

    let a = 'P';
    let b = '🙂';
    let c = '\u{1F642}';  // UTF-8 for smiling face.

    println!("{} {} {}", a, b, c);

    let my_tuple : (char, char, u32);
    my_tuple = (a, b, 10);
    println!("{} {} {}", my_tuple.0, my_tuple.1,  my_tuple.2);

}


fn lets_test_references() {
    // &x borrows a reference to x
    // &T immutable, shared reference. Can have many. Can't write.
    // &mut T mutable, exclusive reference. Can only have one, not even another &T is allowed.
    // (single witer/reader or multiple readers, not both)

    // Boxes: Allocate T in heap.

    // Arrays - Fixed size
    let meus_numeros: [u64; 5] = [1, 2, 3, 4, 1000];
    let letras: [char; 5] = ['p', 'a', 'b', 'l', 'o'];
    let mut um_buffer_de_10 = [10i32; 1024];  // depois posso modificar os valores
    um_buffer_de_10[5] = 20;

    // Vectors - variable size.
    // Quando um novo elemento é adicionado e excede a capacidade do vector:
    //  - um novo é criado com o dobro da capacidade
    //  - antigo é copiado para o novo e o antigo é apagado.
    let mut numeros_no_vect = vec![1, 2, 3];

    println!("Vetor1 criado len {}, capacidade {}", numeros_no_vect.len(), numeros_no_vect.capacity());
    numeros_no_vect.push(10);
    println!("Vetor1 aumentado len {}, capacidade {}", numeros_no_vect.len(), numeros_no_vect.capacity());

    // Podemos criar vetor com capacidade definida.
    let mut vect_com_capacidade: Vec<u64> = Vec::with_capacity(10);
    println!("Vetor2 criado len {}, capacidade {}", vect_com_capacidade.len(), vect_com_capacidade.capacity());
    vect_com_capacidade.push(12);
    vect_com_capacidade.push(45);
    println!("Vetor2 aumentado len {}, capacidade {}", vect_com_capacidade.len(), vect_com_capacidade.capacity());
    for i in 0..9{
        vect_com_capacidade.push(i);
    }
    println!("Vetor2 aumentado len {}, capacidade {}", vect_com_capacidade.len(), vect_com_capacidade.capacity());
    println!("Vect: {:?}", vect_com_capacidade);

    // Slices - are fat pointers - a reference to the starting element and the number of elements.
    // Can't be stored as a variable, only as a reference.
    let um_pedaco = &vect_com_capacidade[1..5];
    println!("Um pedaço {:?}", um_pedaco);
    // ou
    let outro_pedaco: &[u64] = &vect_com_capacidade;
    println!("Um outro pedaço {}", outro_pedaco[2..7].len());



}


