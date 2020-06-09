use std::io;

fn main() {

	println!("Dime una frase...");

	let mut frase = String::new();

	io::stdin()
		.read_line(&mut frase)
		.expect("No pude leer tu frase");

	println!("Tu frase es: {}",frase);

	let caracteres = frase.chars();
	let mut palabras: Vec<String> = Vec::new();

	let mut inicio = 0;

	for (i,car) in caracteres.enumerate() {

		if (car == ' ') | (car == '\r') | (car == '\n') {

			let palabra = frase[inicio..i].to_string();
			if palabra != "" {
				palabras.push(palabra);
			}
			inicio = i + 1;
		}

	}

	println!("Las palabras son: {:?}",palabras);

	let mut nueva_frase = String::from("");

	for palabra in &palabras {

		let nueva_palabra = match &palabra.chars().nth(0) {
			Some('A') => pigger_vowel(&palabra),
			Some('E') => pigger_vowel(&palabra),
			Some('I') => pigger_vowel(&palabra),
			Some('O') => pigger_vowel(&palabra),
			Some('U') => pigger_vowel(&palabra),
			Some('a') => pigger_vowel(&palabra),
			Some('e') => pigger_vowel(&palabra),
			Some('i') => pigger_vowel(&palabra),
			Some('o') => pigger_vowel(&palabra),
			Some('u') => pigger_vowel(&palabra),
			_ => pigger_not_vowel(&palabra),
		};

		if nueva_frase != "" {
			nueva_frase = nueva_frase + " " + &nueva_palabra;
		} else {
			nueva_frase = nueva_frase + &nueva_palabra;
		}

	}

	println!("Tu frase piggeada es: {:?}",nueva_frase);

}

fn pigger_vowel(cadena: &String) -> String {
	cadena[..].to_string() + "-hay"
}

fn pigger_not_vowel(cadena: &String) -> String {
	cadena[1..].to_string() + "-" + &cadena[..1] + "ay"
}
