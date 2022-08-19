mod symbol_table;


/*
Need to know:
- Current Function
- Current Switch Statement

*/
pub struct EnvironmentController{

}

impl EnvironmentController{
    pub fn new() -> Self{
        EnvironmentController {  }
    }
    pub fn build(&mut self,ast: crate::parser::parse_nodes::TranslationUnit){
        todo!()
    }
}