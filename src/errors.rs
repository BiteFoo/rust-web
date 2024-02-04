use core::fmt::Formatter;

pub type  Result<T> = core::result::Result<T,Error>;


#[derive(Debug,Clone)]
pub enum Error {


}

impl std::fmt::Display  for Error{
    fn fmt(&self, f: &mut Formatter ) -> core::result::Result<(),core::fmt::Error> {
        write!(f,"{self:?}")
    }
}
impl  std::error::Error for Error {
    
}