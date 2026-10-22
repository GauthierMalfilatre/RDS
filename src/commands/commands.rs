use std::string;
trait Executer {
    fn execute(&self, args: Vec<string>) -> i32;
}
