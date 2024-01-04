use gtmpl;

fn main() {
   let output = gtmpl::template("Finally, some template {{ . }} in rust", "nuts");
   assert_eq!(&output.unwrap(), "Finally, some template nuts in rust");
   
   println!("Hello, world!");
}
