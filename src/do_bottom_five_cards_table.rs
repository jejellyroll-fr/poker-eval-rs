use std::fs::File;
use std::io::{self, Write};

const HANDVAL_CARD_WIDTH: u32 = 4; // Ajustez selon la définition originale
const STDDECK_N_RANKMASKS: usize = 1 << 13; // Supposons 13 bits pour les rangs

fn bottom_card_func(mask: u32) -> u32 {
    mask.trailing_zeros()
}

fn do_bottom_five_cards_table() -> io::Result<()> {
    let mut file = File::create("t_botfivecards_bis.rs")?;

    writeln!(file, "pub const BOTTOM_FIVE_CARDS_TABLE: [u32; {}] = [", STDDECK_N_RANKMASKS)?;

    for i in 0..STDDECK_N_RANKMASKS as u32 {
        let mut eval: u32 = 0;
        let mut n = i;

        for j in 0..5 {
            if n == 0 {
                eval = 0;
                break;
            }
            let card = bottom_card_func(n);
            eval += card << (j * HANDVAL_CARD_WIDTH);
            n &= !(1 << card);
        }

        if i < (STDDECK_N_RANKMASKS as u32 - 1) {
            writeln!(file, "    0x{:08x},", eval)?;
        } else {
            // Pas de virgule après le dernier élément
            writeln!(file, "    0x{:08x}", eval)?;
        }
    }

    writeln!(file, "];")?;
    Ok(())
}

fn main() -> io::Result<()> {
    do_bottom_five_cards_table()
}
