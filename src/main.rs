use crate::vm::vm::VM;

mod cpu;
mod ram;
mod vm;

fn main() {
    let mut vm = VM::new();

    match vm.execute() {
        Ok(_) => println!("Execution completed successfully."),
        Err(e) => eprintln!("Error occurred during execution: {:?}", e),
    }
}
