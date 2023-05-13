#![allow(non_snake_case)]

use lazy_static::lazy_static;

const PIXELS_WIDTH: usize = 32;
const PIXELS_HEIGHT: usize = 24;
const PIXEL_COUNT: usize = PIXELS_WIDTH * PIXELS_HEIGHT;

const EEPROM_SIZE: usize = 767;

pub struct EepromVars {
    K_Vdd: i16,
    VDD_25: i16,

    T_a: i16,

    pix_os_ref: [i16; PIXEL_COUNT],

    a: [i16; PIXEL_COUNT],

    K_V: [i16; PIXEL_COUNT],

    K_Ta: [i16; PIXEL_COUNT],

    GAIN: i16,

    Ks_Ta: i16,

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

    a_CP_0: i16,
    a_CP_1: i16,

    Off_CP_0: i16,
    Off_CP_1: i16,

    K_V_CP: i16,

    K_Ta_CP: i16,

    TGC: i16,

    Resolution: u16,
}

lazy_static! {
    static ref EEPROM_VARS: EepromVars = restore();
}

static mut EEPROM_RAW: [u16; EEPROM_SIZE] = [0x00; EEPROM_SIZE];

fn read_eeprom() {
    let mut d: [u8; EEPROM_SIZE * 2] = [0x00; EEPROM_SIZE * 2];
    super::read(0x2440, &mut d);

    let mut converted: [u16; EEPROM_SIZE] = [0x00; EEPROM_SIZE];

    for i in 0..EEPROM_SIZE {
        let msb: u16 = d[i * 2 + 0] as u16;
        let lsb: u16 = d[i * 2 + 1] as u16;
        converted[i] = (msb << 8) | lsb;
    }

    unsafe { EEPROM_RAW = converted };
}

fn get_eeprom_val(address: u16) -> u16 {
    let index:usize = (address - 0x2440) as usize;
    return unsafe { EEPROM_RAW[index] };
}

// ----------------------------
// | Temperature Calculations |
// ----------------------------

pub fn evaluate() {
    // We keep Resolution at default, so the coefficient will be just 1
    let Resolution_corr = 1;

    // Calculate Voltage

    // Calculate Ambient temperature

    // Compensate for gain

    // Offset, VDD and Ta

    // Emissivity compensation

    // Gradient compensation

    // Normalize to sensitivity

    // Calculate To
}


pub fn restore() -> EepromVars {
    // Read eeprom data
    read_eeprom();

    // VDD
    let K_Vdd = calc_K_Vdd();
    let VDD_25 = calc_VDD_25();

    // Ta
    let T_a = calc_T_a(VDD_25);

    // Offset
    let pix_os_ref = calc_offset();

    // Sensitivity a (i, j)
    let a = calc_a();

    // K_V (i, j)
    let K_V = calc_K_V();

    // K_Ta (i, j)
    let K_Ta = calc_K_Ta();

    // GAIN
    let gain = calc_gain();

    // Ks_Ta
    let Ks_Ta = calc_Ks_Ta();

    // Corner temperatures
    let Step = calc_Step();
    let CT3 = calc_CT3(Step);
    let CT4 = calc_CT4(Step, CT3);

    // Ks_To
    let mut Ks_To1: i16 = 0;
    let mut Ks_To2: i16 = 0;
    let mut Ks_To3: i16 = 0;
    let mut Ks_To4: i16 = 0;
    calc_Ks_To(&mut Ks_To1, &mut Ks_To2, &mut Ks_To3, &mut Ks_To4);

    // Ranged sensitivity correction
    let Alpha_corr_range1 = calc_Alpha_corr_range1(Ks_To1);
    let Alpha_corr_range2 = calc_Alpha_corr_range2();
    let Alpha_corr_range3 = calc_Alpha_corr_range3(Ks_To2, CT3);
    let Alpha_corr_range4 = calc_Alpha_corr_range4(Ks_To2, Ks_To3, CT3, CT4);

    // Sensitivity a_CP
    let mut a_CP_0: i16 = 0;
    let mut a_CP_1: i16 = 0;
    calc_a_CP(&mut a_CP_0, &mut a_CP_1);

    // Offset of CP
    let mut Off_CP_0: i16 = 0;
    let mut Off_CP_1: i16 = 0;
    calc_Off_CP(&mut Off_CP_0, &mut Off_CP_1);

    // Kv CP
    let K_V_CP = calc_K_V_CP();

    // Kta CP
    let K_Ta_CP = calc_K_Ta_CP();

    // TGC
    let TGC = calc_TGC();

    // Resolution control
    let Resolution = calc_Resolution();

    return EepromVars {
        K_Vdd: K_Vdd,
        VDD_25: VDD_25,

        T_a: T_a,

        pix_os_ref: pix_os_ref,

        a: a,

        K_V: K_V,

        K_Ta: K_Ta,

        GAIN: gain,

        Ks_Ta: Ks_Ta,

        Step: Step,
        CT3: CT3,
        CT4: CT4,

        Ks_To1: Ks_To1,
        Ks_To2: Ks_To2,
        Ks_To3: Ks_To3,
        Ks_To4: Ks_To4,

        Alpha_corr_1: Alpha_corr_range1,
        Alpha_corr_2: Alpha_corr_range2,
        Alpha_corr_3: Alpha_corr_range3,
        Alpha_corr_4: Alpha_corr_range4,

        a_CP_0: a_CP_0,
        a_CP_1: a_CP_1,

        Off_CP_0: Off_CP_0,
        Off_CP_1: Off_CP_1,

        K_V_CP: K_V_CP,

        K_Ta_CP: K_Ta_CP,

        TGC: TGC,

        Resolution: Resolution,
    }
}

fn calc_K_Vdd() -> i16 {
    let mut K_Vdd: i16 = ((get_eeprom_val(0x2433) & 0xFF00) >> 8) as i16;
    if K_Vdd > 127 {
        K_Vdd -= 256;
    }
    return K_Vdd;
}

fn calc_VDD_25() -> i16 {
    let mut VDD_25: i16 = (get_eeprom_val(0x2433) & 0x00FF) as i16;
    VDD_25 = ((VDD_25 - 256) << 5) - (2 as i16).pow(13);
    return VDD_25;
}

fn calc_T_a(VDD_25: i16) -> i16 {
    let mut K_V_PTAT: i16 = ((get_eeprom_val(0x2432) & 0xFC00) >> 10) as i16;
    if K_V_PTAT > 31 {
        K_V_PTAT -= 64;
    }
    K_V_PTAT >>= 12;

    let mut K_T_PTAT: i16 = (get_eeprom_val(0x2432) & 0x3FF) as i16;
    if K_T_PTAT > 511 {
        K_T_PTAT -= 1024;
    }
    K_T_PTAT >>= 3;

    let dV: i16 = (super::read_value(0x072A) as i16 - VDD_25) / K_V_PTAT; // Datasheet just says K_V, i guessed it to be K_V_PTAT

    let V_PTAT_25: i16 = get_eeprom_val(0x2431) as i16;
    //if V_PTAT_25 > 32767 {
    //    V_PTAT_25 -= 65536;
    //}

    let V_PTAT: i16 = super::read_value(0x0720) as i16;
    //if V_PTAT > 32767 {
    //    V_PTAT -= 65536;
    //}

    let V_BE: i16 = super::read_value(0x0700) as i16;
    //if V_BE > 32767 {
    //    V_BE -= 65536;
    //}

    let Alpha_PTAT_EE: i16 = ((get_eeprom_val(0x2410) & 0xF000) >> 12) as i16;
    let Alpha_PTAT: i16 = (Alpha_PTAT_EE >> 2) + 8;

    let V_PTAT_art: i16 = (V_PTAT / (V_PTAT * Alpha_PTAT + V_BE)) << 18;

    let mut T_a: i16 = V_PTAT_art / (1 + K_V_PTAT * dV);
    T_a -= V_PTAT_25;
    T_a /= K_T_PTAT;
    T_a += 25;

    return T_a;
}

fn calc_offset() -> [i16; PIXEL_COUNT] {
    let offset_avg: i16 = get_eeprom_val(0x2411) as i16;
    //if offset_avg > 32767 {
    //    offset_avg -= 65536;
    //}

    // OCC row i
    let mut OCC_row: [i16; PIXELS_HEIGHT] = [0x00; PIXELS_HEIGHT];
    for row in 0..PIXELS_HEIGHT/4 {
        let address: u16 = (0x2412 + row) as u16;

        OCC_row[row * 4 + 0] = ((get_eeprom_val(address) & 0x000F) >> 0) as i16;
        OCC_row[row * 4 + 1] = ((get_eeprom_val(address) & 0x00F0) >> 4) as i16;
        OCC_row[row * 4 + 2] = ((get_eeprom_val(address) & 0x0F00) >> 8) as i16;
        OCC_row[row * 4 + 3] = ((get_eeprom_val(address) & 0xF000) >> 12) as i16;

        if OCC_row[row * 4 + 0] > 7 { OCC_row[row * 4 + 0] -= 16 }
        if OCC_row[row * 4 + 1] > 7 { OCC_row[row * 4 + 1] -= 16 }
        if OCC_row[row * 4 + 2] > 7 { OCC_row[row * 4 + 2] -= 16 }
        if OCC_row[row * 4 + 3] > 7 { OCC_row[row * 4 + 3] -= 16 }
    }

    // OCC scale row
    let OCC_scale_row: u16 = (get_eeprom_val(0x2410) & 0x0F00) as u16 >> 8;

    // OCC column
    let mut OCC_column: [i16; PIXELS_WIDTH] = [0x00; PIXELS_WIDTH];
    for column in 0..PIXELS_WIDTH/4 {
        let address: u16 = (0x2418 + column) as u16;

        OCC_column[column * 4 + 0] = ((get_eeprom_val(address) & 0x000F) >> 0) as i16;
        OCC_column[column * 4 + 1] = ((get_eeprom_val(address) & 0x00F0) >> 4) as i16;
        OCC_column[column * 4 + 2] = ((get_eeprom_val(address) & 0x0F00) >> 8) as i16;
        OCC_column[column * 4 + 3] = ((get_eeprom_val(address) & 0xF000) >> 12) as i16;

        if OCC_column[column * 4 + 0] > 7 { OCC_column[column * 4 + 0] -= 16 }
        if OCC_column[column * 4 + 1] > 7 { OCC_column[column * 4 + 1] -= 16 }
        if OCC_column[column * 4 + 2] > 7 { OCC_column[column * 4 + 2] -= 16 }
        if OCC_column[column * 4 + 3] > 7 { OCC_column[column * 4 + 3] -= 16 }
    }

    // OCC scale column
    let OCC_scale_column: u16 = (get_eeprom_val(0x2410) & 0x00F0) as u16 >> 4;

    // offset
    let mut offset: [i16; PIXEL_COUNT] = [0x00; PIXEL_COUNT];
    for i in 0..PIXEL_COUNT {
        let address: u16 = (0x2440 + i) as u16;

        offset[i] = ((get_eeprom_val(address) & 0xFC00) >> 10) as i16;
        if offset[i] > 31 {
            offset[i] -= 64;
        }
    }

    // OCC scale remnant
    let OCC_scale_remnant: u16 = get_eeprom_val(0x2410) as u16 & 0x000F;

    let mut pix_os_ref: [i16; PIXEL_COUNT] = [0x00; PIXEL_COUNT];
    for i in 0..PIXELS_HEIGHT {
        for j in 0..PIXELS_WIDTH {
            let index = i * PIXELS_WIDTH + j;
            pix_os_ref[index] = offset_avg;
            pix_os_ref[index] += OCC_row[i] * (2 as i16).pow(OCC_scale_row.into());
            pix_os_ref[index] += OCC_column[j] * (2 as i16).pow(OCC_scale_column.into());
            pix_os_ref[index] += offset[index] * (2 as i16).pow(OCC_scale_remnant.into());
        }
    }
    return pix_os_ref;
}

fn calc_a() -> [i16; PIXEL_COUNT] {
    let a_reference: i16 = get_eeprom_val(0x2421) as i16;

    let a_scale: i16 = ((get_eeprom_val(0x2420) & 0xF000) >> 12) as i16 + 30;

    let mut ACC_row: [i16; PIXELS_HEIGHT] = [0x00; PIXELS_HEIGHT];
    for row in 0..PIXELS_HEIGHT/4 {
        let address: u16 = 0x2422 + row as u16;

        ACC_row[row * 4 + 0] = ((get_eeprom_val(address) & 0x000F) >> 0) as i16;
        ACC_row[row * 4 + 1] = ((get_eeprom_val(address) & 0x00F0) >> 4) as i16;
        ACC_row[row * 4 + 2] = ((get_eeprom_val(address) & 0x0F00) >> 8) as i16;
        ACC_row[row * 4 + 3] = ((get_eeprom_val(address) & 0xF000) >> 12) as i16;

        if ACC_row[row * 4 + 0] > 7 { ACC_row[row * 4 + 0] -= 16 }
        if ACC_row[row * 4 + 1] > 7 { ACC_row[row * 4 + 1] -= 16 }
        if ACC_row[row * 4 + 2] > 7 { ACC_row[row * 4 + 2] -= 16 }
        if ACC_row[row * 4 + 3] > 7 { ACC_row[row * 4 + 3] -= 16 }
    }

    let ACC_scale_row: u16 = (get_eeprom_val(0x2420) & 0x0F00) as u16 >> 8;

    let mut ACC_column: [i16; PIXELS_WIDTH] = [0x00; PIXELS_WIDTH];
    for column in 0..PIXELS_WIDTH/4 {
        let address: u16 = 0x2428 + column as u16;

        ACC_column[column * 4 + 0] = ((get_eeprom_val(address) & 0x000F) >> 0) as i16;
        ACC_column[column * 4 + 1] = ((get_eeprom_val(address) & 0x00F0) >> 4) as i16;
        ACC_column[column * 4 + 2] = ((get_eeprom_val(address) & 0x0F00) >> 8) as i16;
        ACC_column[column * 4 + 3] = ((get_eeprom_val(address) & 0xF000) >> 12) as i16;

        if ACC_column[column * 4 + 0] > 7 { ACC_column[column * 4 + 0] -= 16 }
        if ACC_column[column * 4 + 1] > 7 { ACC_column[column * 4 + 1] -= 16 }
        if ACC_column[column * 4 + 2] > 7 { ACC_column[column * 4 + 2] -= 16 }
        if ACC_column[column * 4 + 3] > 7 { ACC_column[column * 4 + 3] -= 16 }
    }

    let ACC_scale_column: u16 = (get_eeprom_val(0x2420) & 0x00F0) as u16 >> 4;

    let mut a_pixel: [i16; PIXEL_COUNT] = [0x00; PIXEL_COUNT];
    for i in 0..PIXEL_COUNT {
        let address: u16 = 0x2440 + i as u16;

        a_pixel[i] = ((get_eeprom_val(address) & 0x03F0) >> 4) as i16;
        if a_pixel[i] > 31 {
            a_pixel[i] -= 64;
        }
    }

    let ACC_scale_remnant: u16 = get_eeprom_val(0x2420) as u16 & 0x000F;

    let mut a: [i16; PIXEL_COUNT] = [0x00; PIXEL_COUNT];
    for i in 0..PIXELS_HEIGHT {
        for j in 0..PIXELS_WIDTH {
            let index = i * PIXELS_WIDTH + j;

            a[index] = a_reference;
            a[index] += ACC_row[i] * (2 as i16).pow(ACC_scale_row as u32);
            a[index] += ACC_column[j] * (2 as i16).pow(ACC_scale_column as u32);
            a[index] += a_pixel[index] * (2 as i16).pow(ACC_scale_remnant as u32);
            a[index] /= (2 as i16).pow(a_scale as u32);
        }
    }
    return a;
}

fn calc_K_V() -> [i16; PIXEL_COUNT] {
    let K_V_scale: u16 = (get_eeprom_val(0x2438) & 0x0F00) as u16 >> 8;

    let mut K_V: [i16; PIXEL_COUNT] = [0x00; PIXEL_COUNT];
    // EVEN EVEN
    for i in (0..PIXELS_HEIGHT).step_by(2) {
        for j in (0..PIXELS_WIDTH).step_by(2) {
            let index = i * PIXELS_WIDTH + j;
            K_V[index] = ((get_eeprom_val(0x2434) & 0xF000) >> 12) as i16;
        }
    }

    // ODD EVEN
    for i in (1..PIXELS_HEIGHT).step_by(2) {
        for j in (0..PIXELS_WIDTH).step_by(2) {
            let index = i * PIXELS_WIDTH + j;
            K_V[index] = ((get_eeprom_val(0x2434) & 0x0F00) >> 8) as i16;
        }
    }

    // EVEN ODD
    for i in (0..PIXELS_HEIGHT).step_by(2) {
        for j in (1..PIXELS_WIDTH).step_by(2) {
            let index = i * PIXELS_WIDTH + j;
            K_V[index] = ((get_eeprom_val(0x2434) & 0x00F0) >> 4) as i16;
        }
    }

    // ODD ODD
    for i in (1..PIXELS_HEIGHT).step_by(2) {
        for j in (1..PIXELS_WIDTH).step_by(2) {
            let index = i * PIXELS_WIDTH + j;
            K_V[index] = ((get_eeprom_val(0x2434) & 0x000F) >> 0) as i16;
        }
    }

    for i in 0..PIXEL_COUNT {
        if K_V[i] > 7 {
            K_V[i] -= 16;
        }

        K_V[i] /= (2 as i16).pow(K_V_scale as u32);
    }

    return K_V;
}

fn calc_K_Ta() -> [i16; PIXEL_COUNT] {
    let mut K_Ta_EE: [i16; PIXEL_COUNT] = [0x00; PIXEL_COUNT];

    for i in 0..PIXEL_COUNT {
        let address: u16 = 0x2440 + i as u16;

        K_Ta_EE[i] = ((get_eeprom_val(address) & 0x000E) >> 1) as i16;
        if K_Ta_EE[i] > 3 {
            K_Ta_EE[i] -= 8;
        }
    }

    let mut K_Ta_RC_EE: [i16; PIXEL_COUNT] = [0x00; PIXEL_COUNT];

    // EVEN EVEN
    for i in (0..PIXELS_HEIGHT).step_by(2) {
        for j in (0..PIXELS_WIDTH).step_by(2) {
            let index = i * PIXELS_WIDTH + j;
            K_Ta_RC_EE[index] = ((get_eeprom_val(0x2436) & 0xFF00) >> 8) as i16;
        }
    }

    // ODD EVEN
    for i in (1..PIXELS_HEIGHT).step_by(2) {
        for j in (0..PIXELS_WIDTH).step_by(2) {
            let index = i * PIXELS_WIDTH + j;
            K_Ta_RC_EE[index] = ((get_eeprom_val(0x2436) & 0x00FF) >> 0) as i16;
        }
    }

    // EVEN ODD
    for i in (0..PIXELS_HEIGHT).step_by(2) {
        for j in (1..PIXELS_WIDTH).step_by(2) {
            let index = i * PIXELS_WIDTH + j;
            K_Ta_RC_EE[index] = ((get_eeprom_val(0x2437) & 0xFF00) >> 8) as i16;
        }
    }

    // ODD ODD
    for i in (1..PIXELS_HEIGHT).step_by(2) {
        for j in (1..PIXELS_WIDTH).step_by(2) {
            let index = i * PIXELS_WIDTH + j;
            K_Ta_RC_EE[index] = ((get_eeprom_val(0x2437) & 0x00FF) >> 0) as i16;
        }
    }

    for i in 0..PIXEL_COUNT {
        if K_Ta_RC_EE[i] > 127 {
            K_Ta_RC_EE[i] -= 256;
        }
    }

    let K_Ta_scale1: u16 = ((get_eeprom_val(0x2438) & 0x00F0) as u16 >> 4) + 8;

    let K_Ta_scale2: u16 = (get_eeprom_val(0x2438) & 0x000F) as u16;

    let mut K_Ta: [i16; PIXEL_COUNT] = [0x00; PIXEL_COUNT];

    for i in 0..PIXEL_COUNT {
        K_Ta[i] = K_Ta_RC_EE[i];
        K_Ta[i] += K_Ta_EE[i] * (2 as i16).pow(K_Ta_scale2 as u32);
        K_Ta[i] /= (2 as i16).pow(K_Ta_scale1 as u32);
    }

    return K_Ta;
}

fn calc_gain() -> i16 {
    let gain: i16 = get_eeprom_val(0x2430) as i16;
    //if gain > 32767 {
    //    gain -= 65536;
    //}
    return gain;
}

fn calc_Ks_Ta() -> i16 {
    let mut Ks_Ta_EE: i16 = ((get_eeprom_val(0x243C) & 0xFF00) >> 8) as i16;
    if Ks_Ta_EE > 127 {
        Ks_Ta_EE -= 256;
    }

    let Ks_Ta: i16 = Ks_Ta_EE >> 13;
    return Ks_Ta;
}

fn calc_Step() -> i16 {
    return ((get_eeprom_val(0x243F) & 0x3000) >> 12) as i16 * 10;
}

fn calc_CT3(Step: i16) -> i16 {
    return ((get_eeprom_val(0x243F) & 0x00F0) >> 4) as i16 * Step;
}

fn calc_CT4(Step: i16, CT3: i16) -> i16 {
    return ((get_eeprom_val(0x243F) & 0x0F00) >> 8) as i16 * Step + CT3;
}

fn calc_Ks_To(Ks_To1: &mut i16, Ks_To2: & mut i16, Ks_To3: &mut i16, Ks_To4: &mut i16) {
    let Ks_To_scale: u16 = get_eeprom_val(0x243F) as u16 & 0x000F + 8;

    let mut Ks_To1_EE: i16 = (get_eeprom_val(0x243D) & 0x00FF) as i16;
    if Ks_To1_EE > 127 { Ks_To1_EE -= 256 }
    *Ks_To1 = Ks_To1_EE / (2 as i16).pow(Ks_To_scale as u32);

    let mut Ks_To2_EE: i16 = ((get_eeprom_val(0x243D) & 0xFF00) >> 8) as i16;
    if Ks_To2_EE > 127 { Ks_To2_EE -= 256 }
    *Ks_To2 = Ks_To2_EE / (2 as i16).pow(Ks_To_scale as u32);

    let mut Ks_To3_EE: i16 = (get_eeprom_val(0x243E) & 0x00FF) as i16;
    if Ks_To3_EE > 127 { Ks_To3_EE -= 256 }
    *Ks_To3 = Ks_To3_EE / (2 as i16).pow(Ks_To_scale as u32);

    let mut Ks_To4_EE: i16 = ((get_eeprom_val(0x243E) & 0xFF00) >> 8) as i16;
    if Ks_To4_EE > 127 { Ks_To4_EE -= 256 }
    *Ks_To4 = Ks_To4_EE / (2 as i16).pow(Ks_To_scale as u32);
}

fn calc_Alpha_corr_range1(Ks_To1: i16) -> i16 {
    // TODO
    // The inversion seems weird
    return 1 / (1 + Ks_To1 * 40);
}

fn calc_Alpha_corr_range2() -> i16 {
    return 1;
}

fn calc_Alpha_corr_range3(Ks_To2: i16, CT3: i16) -> i16 {
    return 1 + Ks_To2 * CT3;
}

fn calc_Alpha_corr_range4(Ks_To2: i16, Ks_To3: i16, CT3: i16, CT4: i16) -> i16 {
    return (1 + Ks_To2 * CT3) * (1 + Ks_To3 * (CT4 - CT3));
}

fn calc_a_CP(a_CP_0: &mut i16, a_CP_1: &mut i16) {
    let a_scale_CP = ((get_eeprom_val(0x2420) & 0xF000) >> 12) as i16 + 27;
    let mut CP_P1_P0_ratio = ((get_eeprom_val(0x2439) & 0xFC00) >> 10) as i16;
    if CP_P1_P0_ratio > 31 {
        CP_P1_P0_ratio -= 64;
    }

    *a_CP_0 = ((get_eeprom_val(0x2439) & 0x03FF)) as i16 / (2 as i16).pow(a_scale_CP as u32);
    *a_CP_1 = *a_CP_0 * (1 + (CP_P1_P0_ratio >> 7));
}

fn calc_Off_CP(Off_CP_0: &mut i16, Off_CP_1: &mut i16) {
    *Off_CP_0 = (get_eeprom_val(0x243A) & 0x03FF) as i16;
    if *Off_CP_0 > 511 {
        *Off_CP_0 -= 1024;
    }

    let mut Off_CP_1_delta: i16 = ((get_eeprom_val(0x243A) & 0xFC00) >> 10) as i16;
    if Off_CP_1_delta > 31 {
        Off_CP_1_delta -= 64;
    }

    *Off_CP_1 = *Off_CP_0 + Off_CP_1_delta;
}

fn calc_K_V_CP() -> i16 {
    let K_V_Scale: u16 = (get_eeprom_val(0x2438) & 0x0F00) as u16 >> 8;

    let mut K_V_CP_EE: i16 = ((get_eeprom_val(0x243B) & 0xFF00) >> 8) as i16;
    if K_V_CP_EE > 127 {
        K_V_CP_EE -= 256;
    }

    let K_V_CP: i16 = K_V_CP_EE / (2 as i16).pow(K_V_Scale as u32);
    return K_V_CP;
}

fn calc_K_Ta_CP() -> i16 {
    let K_Ta_scale_1: u16 = ((get_eeprom_val(0x2438) & 0x00F0) as u16 >> 4) + 8;

    let mut K_Ta_CP_EE: i16 = (get_eeprom_val(0x243B) & 0x00FF) as i16;
    if K_Ta_CP_EE > 127 {
        K_Ta_CP_EE -= 256;
    }

    let K_Ta_CP = K_Ta_CP_EE / (2 as i16).pow(K_Ta_scale_1 as u32);
    return K_Ta_CP;
}

fn calc_TGC() -> i16 {
    let mut TGC_EE: i16 = (get_eeprom_val(0x243C) & 0x00FF) as i16;
    if TGC_EE > 127 {
        TGC_EE -= 256;
    }

    let TGC = TGC_EE >> 5;
    return TGC;
}

fn calc_Resolution() -> u16 {
    return (get_eeprom_val(0x2438) & 0x3000) as u16 >> 12;
}
