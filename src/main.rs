mod piece;

use piece::pawn::Pawn;

fn main() {
    println!("Chess Game");
    let mut pawn: Pawn = Pawn::new();
    pawn.display();
    pawn.shift(2, 3);
    pawn.display()
}
