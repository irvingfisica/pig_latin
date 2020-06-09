use std::io;

fn main() {

	println!("Dime una frase...");

	let mut frase = String::new();

	io::stdin()
		.read_line(&mut frase)
		.expect("No pude leer tu frase");

	println!("Tu frase es {}",frase);

	let caracteres = frase.chars();
	let mut palabras: Vec<String> = Vec::new();

	println!("{:?}",caracteres);

	let mut inicio = 0;

	for (i,car) in caracteres.enumerate() {

		println!("{},{}",i,car );

		if (car == ' ') | (car == '\r') | (car == '\n') {

			let palabra = frase[inicio..i].to_string();
			if palabra != "" {
				palabras.push(palabra);
			}
			inicio = i + 1;
		}

	}

	println!("{:?}",palabras);

}
