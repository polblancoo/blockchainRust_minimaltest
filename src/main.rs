mod models;
fn main() {
    let difficulty = 1;
    let mut blockchain = models::blockchain::Blockchain::new(difficulty);
//Habria que poner alguna regla de llamada para el siguioente block
//como un consenso de la red > %52 u otra forma 
for _i in 1.. 20 {
    models::blockchain::Blockchain::add_block(&mut blockchain);
    //change de dificulty 
    if _i < 7{
        blockchain.difficulty = _i
    }else {
        blockchain.difficulty = 1

    }

}
    
}
