use rand::{ thread_rng, seq::SliceRandom };

const STATES: [u8; 8] = [
    0b00000001,
    0b00000010,
    0b00000100,
    0b00001000,
    0b00010000,
    0b00100000,
    0b01000000,
    0b10000000
];
const STATE_NEIGHBORS: [u8; 8] = [
    0b00000011,
    0b00000111,
    0b00001110,
    0b00011100,
    0b00111000,
    0b01110000,
    0b11100000,
    0b11000000
];
const STATE_ALL: u8 = 0b11111111;
const STATE_1: u8 = STATES[0];
const STATE_2: u8 = STATES[1];
const STATE_3: u8 = STATES[2];
const STATE_4: u8 = STATES[3];
const STATE_5: u8 = STATES[4];
const STATE_6: u8 = STATES[5];
const STATE_7: u8 = STATES[6];
const STATE_8: u8 = STATES[7];

const ENTROPY_LOOKUP_TABLE: [u8; 256] = generate_entropy_lookup_table();
const fn generate_entropy_lookup_table() -> [u8; 256] {
    let mut result: [u8; 256] = [0; 256];
    let mut i: u8 = 0;
    loop {
        let mut val: u8 = i;
        let mut count: u8 = 0;
        while val != 0 {
            count += val & 1;
            val >>= 1;
        }
        result[i as usize] = count;
        i += 1;
        if i == 255 {
            break;
        }
    }
    result[255] = 8;
    return result;
}

#[derive(Copy, Clone)]
pub struct Tile {
    state: u8
}

impl Default for Tile {
    fn default() -> Self {
        Tile {
            state: STATE_ALL
        }
    }
}

impl Tile {
    pub fn entropy(&self) -> u8 {
        return ENTROPY_LOOKUP_TABLE[self.state as usize];
    }

    pub fn collapse(&mut self) {
        let mut available_states: Vec<u8> = Vec::new();
        for state in STATES {
            if self.state & state != 0 {
                available_states.push(state);
            }
        }

        let mut rng = thread_rng();
        if let Some(state) = available_states.choose(&mut rng) {
            self.state = *state;
        }
    }

    pub fn propogate(&mut self, neighbor: Tile) -> bool {
        let pre_change: u8 = self.state;
        for i in 0..8 {
            if neighbor.state & STATE_NEIGHBORS[i] == 0 {
                self.state &= !STATES[i];
            }
        }
        return pre_change != self.state;
    }

    pub fn print_tile(&self) -> char {
        match self.state {
            STATE_1 => return '1',
            STATE_2 => return '2',
            STATE_3 => return '3',
            STATE_4 => return '4',
            STATE_5 => return '5',
            STATE_6 => return '6',
            STATE_7 => return '7',
            STATE_8 => return '8',
            _default => return ' '
        }
    }

    pub fn print_state(&self) -> String {
        return format!("{:08b}", self.state);
    }
}