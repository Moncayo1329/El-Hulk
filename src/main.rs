use std::io;

#[derive(Debug)]

struct FoodEntry {
    name: String,
    calories:u32,
}






fn main() {
let mut entries: Vec<FoodEntry> = Vec::new();


loop {

println!("\n1. Agregar comida");
println!("2. Ver total de calorias");
println!("3. Ver comidas registradas");
println!("4. Salir");

let mut choice = String::new();

io::stdin().read_line(&mut choice).unwrap();

match choice.trim(){

"1" => {
    let mut name = String::new();
    let mut cal_str = String::new();


 println!("Nombre del alimento");
 io::stdin().read_line(&mut name).unwrap();

 println!("Cantidad de calorias:");
 io::stdin().read_line(&mut cal_str).unwrap();


 let calories: u32 = match cal_str.trim().parse(){
   
    Ok(num) => num,
    Err(_) => {
        println!("Numero invalido");
        continue;
    }
 };

 entries.push(FoodEntry {

name:name.trim().to_string(),
calories,
 });


println!("Comida agregada!");


}

"2" => {
    let total:u32 = entries.iter().map(|e| e.calories).sum();
    println!("Total de calorias: {}", total);
}

"3" => {
    for (_i,entry) in entries.iter().enumerate(){
        println!("{}. {} - {} cal",1 + 1, entry.name, entry.calories)
    }
}

"4" => break,

_ => println!("Opcion invalida")



}

}

}
