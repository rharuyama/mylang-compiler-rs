use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
	.read_line(&mut input)
	.expect("Error -- in main");

    let tokens: Vec<&str> = input.split_whitespace().collect();

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    for i in 0..tokens.len(){
	match tokens[i] {
	    "+" => {
		println!("   pop rdi");
		println!("   pop rax");
		println!("   add rax, rdi");
		println!("   push rax");
		println!("");
	    },
	    "-" => {
		println!("   pop rdi");
		println!("   pop rax");
		println!("   sub rax, rdi");
		println!("   push rax");
		println!("");
	    },
	    "*" => {
		println!("   pop rdi");
		println!("   pop rax");
		println!("   imul rax, rdi");
		println!("   push rax");
		println!("");
	    },
	    "/" => {
		println!("   pop rdi");
		println!("   pop rax");
		println!("   cqo");
		println!("   idiv rdi");
		println!("   push rax");
		println!("");
	    },
	    num => {
		println!("   push {}", num);
		println!("")
	    } ,
	}
    }

    println!("   pop rax");
    println!("   ret")
}

