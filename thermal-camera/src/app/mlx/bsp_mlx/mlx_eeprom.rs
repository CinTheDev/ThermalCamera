use lazy_static::lazy_static;
use power_of_two::power_of_two;

const PIXELS_WIDTH: usize = 32;
const PIXELS_HEIGHT: usize = 24;
const PIXEL_COUNT: usize = PIXELS_WIDTH * PIXELS_HEIGHT;

const EEPROM_SIZE: usize = 767;

static mut EEPROM_RAW: [i16; EEPROM_SIZE] = [0x00; EEPROM_SIZE];

fn read_eeprom() {
    let mut d: [u8; EEPROM_SIZE * 2] = [0x00; EEPROM_SIZE * 2];
    super::read(0x2440, &mut d);

    let mut converted: [i16; EEPROM_SIZE] = [0x00; EEPROM_SIZE];

    for i in 0..EEPROM_SIZE {
        let msb: i16 = d[i * 2 + 0] as i16;
        let lsb: i16 = d[i * 2 + 1] as i16;
        converted[i] = (msb << 8) | lsb;
    }

    unsafe { EEPROM_RAW = converted };
}

fn get_eeprom_val(address: u16) -> i16 {
    let index:usize = (address - 0x2440) as usize;
    return unsafe { EEPROM_RAW[index] };
}

struct eeprom_vars {
    K_Vdd: i16,
    VDD_25: i16,

    T_a: i16,

    pix_os_ref: [i16; PIXEL_COUNT],

    a: [i16; PIXEL_COUNT],

    Kv: [i16; PIXEL_COUNT],

    Kta: [i16; PIXEL_COUNT],

    GAIN: i16,

    KsTa: i16,

    Step: i16,
    CT3: i16,
    CT4: i16,

    Ks_To1: i16,
    Ks_To2: i16,
    Ks_To3: i16,
    Ks_To4: i16,

    Alpha_corr_1: i16,
    Alpha_corr_2: i16,
    Alpha_corr_3: i16,
    Alpha_corr_4: i16,

    a_cp_0: i16,
    a_cp_1: i16,

    off_cp_0: i16,
    off_cp_1: i16,

    Kv_cp: i16,

    K_Ta_cp: i16,

    TGC: i16,

    Resolution: u16,
}

lazy_static! {
    static ref EEPROM_VARS: eeprom_vars = restore();
}

pub fn restore() -> eeprom_vars {
    // Read eeprom data
    read_eeprom();

    // VDD
    let mut K_Vdd: i16 = (get_eeprom_val(0x2433) & 0xFF00) / power_of_two!(8) as i16;
    if K_Vdd > 127 {
        K_Vdd -= 256;
    }
    let mut VDD_25: i16 = get_eeprom_val(0x2433) & 0x00FF;
    VDD_25 = (VDD_25 - 256) * power_of_two!(5) as i16 - power_of_two!(13) as i16;

    // Ta
    let mut K_V_PTAT: i16 = (get_eeprom_val(0x2432) & 0xFC00) / power_of_two!(10) as i16;
    if K_V_PTAT > 31 {
        K_V_PTAT -= 64;
    }
    K_V_PTAT /= power_of_two!(12) as i16;

    let mut K_T_PTAT: i16 = get_eeprom_val(0x2432) & 0x3FF;
    if K_T_PTAT > 511 {
        K_T_PTAT -= 1024;
    }
    K_T_PTAT /= power_of_two!(3) as i16;

    let dV: i16 = (super::read_value(0x072A) as i16 - VDD_25) / K_V_PTAT; // Datasheet just says K_V, i guessed it to be K_V_PTAT

    let mut V_PTAT_25: i16 = get_eeprom_val(0x2431);
    if V_PTAT_25 > 32767 {
        V_PTAT_25 -= 65536;
    }

    let mut V_PTAT: i16 = super::read_value(0x0720) as i16;
    if V_PTAT > 32767 {
        V_PTAT -= 65536;
    }

    let mut V_BE: i16 = super::read_value(0x0700) as i16;
    if V_BE > 32767 {
        V_BE -= 65536;
    }

    let Alpha_PTAT_EE: i16 = (get_eeprom_val(0x2410) & 0xF000) / power_of_two!(12) as i16;
    let Alpha_PTAT: i16 = Alpha_PTAT_EE / power_of_two!(2) as i16 + 8;

    let V_PTAT_art: i16 = (V_PTAT / (V_PTAT * Alpha_PTAT + V_BE)) * power_of_two!(18) as i16;

    let mut T_a: i16 = V_PTAT_art / (1 + K_V_PTAT * dV);
    T_a -= V_PTAT_25;
    T_a /= K_T_PTAT;
    T_a += 25;

    // Offset

    // Sensitivity a (i, j)

    // Kv (i, j)

    // Kta (i, j)

    // GAIN

    // KsTa

    // Corner temperatures

    // KsTo

    // Ranged sensitivity correction

    // Sensitivity a_CP

    // Offset of CP

    // Kv CP

    // Kta CP

    // TGC

    // Resolution control

    return eeprom_vars {
        K_Vdd: K_Vdd,
        VDD_25: VDD_25,

        T_a: T_a,
    }
}

fn calibrate() {
    // Calculate Voltage

    // Calculate Ambient temperature

    // Compensate for gain

    // Offset, VDD and Ta

    // Emissivity compensation

    // Gradient compensation

    // Normalize to sensitivity

    // Calculate To
}
