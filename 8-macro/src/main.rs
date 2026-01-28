use assignment8_macro::*;
use btreemap_macro::btreemap as procedural_btreemap;


fn main() {
let declarative_map = btreemap! {
"a" => 1,
"b" => 2,
};
println!("Declarative: {:?}", declarative_map);


let procedural_map = procedural_btreemap!("x", 10, "y", 20);
println!("Procedural: {:?}", procedural_map);
}
//Для pull request
