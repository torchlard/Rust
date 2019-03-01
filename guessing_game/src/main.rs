mod sound {
  pub mod instrument {
    fn guitar(){
      println!("guitar!");
    }
  }
  mod voice {

  }
}

fn main(){

  crate::sound::instrument::guitar();
  sound::instrument::guitar();

}














