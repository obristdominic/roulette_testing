use crate::odds;

pub fn martingale(mut bet: u16, mut pot: u16) {
  while pot > 0 {
    println!("Pot: {}\nBet: {}\n", pot, bet);
    if odds::win_or_loss(486) {
      pot += bet;
      bet = 1;
    } else {
      pot -= bet;
      bet *= 2;

      if bet > pot {
        bet = pot;
      }
    }
  }
}
