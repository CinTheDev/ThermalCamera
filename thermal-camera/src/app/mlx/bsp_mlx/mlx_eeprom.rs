#![allow(non_snake_case)]

use lazy_static::lazy_static;

const PIXELS_WIDTH: usize = 32;
const PIXELS_HEIGHT: usize = 24;
const PIXEL_COUNT: usize = PIXELS_WIDTH * PIXELS_HEIGHT;

const EEPROM_SIZE: usize = 816;

pub struct EepromVars {
    K_Vdd: i32,
    VDD_25: i32,

    K_V_PTAT: f32,
    K_T_PTAT: f32,
    V_PTAT_25: i32,
    Alpha_PTAT: f32,

    pix_os_ref: [i32; PIXEL_COUNT],

    a: [f32; PIXEL_COUNT],

    K_V: [f32; PIXEL_COUNT],

    K_Ta: [f32; PIXEL_COUNT],

    GAIN: i32,

    Ks_Ta: f32,

    Step: i32,
    CT3: i32,
    CT4: i32,

    Ks_To: (f32, f32, f32, f32),

    Alpha_corr: (f32, f32, f32, f32),

    a_CP: (f32, f32),

    Off_CP: (i32, i32),

    K_V_CP: f32,

    K_Ta_CP: f32,

    TGC: f32,

    Resolution: u16,

    pattern: [u16; PIXEL_COUNT],
}

lazy_static! {
    static ref EEPROM_VARS: EepromVars = restore();
}

static mut EEPROM_RAW: [u16; EEPROM_SIZE] = [0x00; EEPROM_SIZE];

fn read_eeprom() {
    let mut d: [u8; EEPROM_SIZE * 2] = [0x00; EEPROM_SIZE * 2];
    super::read(0x2410, &mut d);

    let mut converted: [u16; EEPROM_SIZE] = [0x00; EEPROM_SIZE];

    for i in 0..EEPROM_SIZE {
        let msb: u16 = d[i * 2 + 0] as u16;
        let lsb: u16 = d[i * 2 + 1] as u16;
        converted[i] = (msb << 8) | lsb;
    }

    unsafe { EEPROM_RAW = converted };
}

fn get_eeprom_val(address: u16) -> u16 {
    let index:usize = (address - 0x2410) as usize;
    return unsafe { EEPROM_RAW[index] };
}

// ----------------------------
// | Temperature Calculations |
// ----------------------------

pub fn evaluate(pix_data: [u16; PIXEL_COUNT]) -> [f32; PIXEL_COUNT] {
    const EMISSIVITY: f32 = 1.0;

    let Resolution_corr: f32 = 2_f32.powi(EEPROM_VARS.Resolution as i32) / 2_f32.powi((super::read_value(0x800D) as i32 & 0x0C00) >> 10);

    // Calculate Voltage
    let V_dd = calc_V_dd(Resolution_corr);

    // Calculate Ambient temperature
    let T_a = calc_T_a();

    // Compensate for gain
    let K_gain = calc_K_gain();

    let pix_gain = calc_pix_gain(K_gain, pix_data);

    // Offset, VDD and Ta
    let pix_os = calc_pix_os(pix_gain, V_dd, T_a);

    // Emissivity compensation
    let V_IR_Em_compensated = calc_V_IR_Em_compensated(pix_os, EMISSIVITY);

    // CP gain compensation
    let pix_OS_CP_SP = calc_pix_OS_CP_SPX(V_dd, T_a, K_gain);

    // Gradient compensation
    let V_IR_compensated = calc_V_IR_compensated(V_IR_Em_compensated, pix_OS_CP_SP);

    // Normalize to sensitivity
    let a_comp = calc_a_comp(T_a);

    // Calculate To
    let T_o = calc_T_o(EMISSIVITY, T_a, V_IR_compensated, a_comp);

    // TODO: do additional temperature ranges
    return T_o;
}


pub fn restore() -> EepromVars {
    // Read eeprom data
    read_eeprom();

    // VDD
    let K_Vdd = restore_K_Vdd();
    let VDD_25 = restore_VDD_25();

    // Ta
    let mut K_V_PTAT: f32 = 0.0;
    let mut K_T_PTAT: f32 = 0.0;
    let mut V_PTAT_25: i32 = 0;
    let mut Alpha_PTAT: f32 = 0.0;
    restore_T_a(&mut K_V_PTAT, &mut K_T_PTAT, &mut V_PTAT_25, &mut Alpha_PTAT);

    // Offset
    let pix_os_ref = restore_offset();

    // Sensitivity a (i, j)
    let a = restore_a();

    // K_V (i, j)
    let K_V = restore_K_V();

    // K_Ta (i, j)
    let K_Ta = restore_K_Ta();

    // GAIN
    let GAIN = restore_gain();

    // Ks_Ta
    let Ks_Ta = restore_Ks_Ta();

    // Corner temperatures
    let Step = restore_Step();
    let CT3 = restore_CT3(Step);
    let CT4 = restore_CT4(Step, CT3);

    // Ks_To
    let Ks_To = restore_Ks_To();

    // Ranged sensitivity correction
    let Alpha_corr = restore_Alpha_corr(Ks_To, CT3, CT4);

    // Sensitivity a_CP
    let a_CP = restore_a_CP();

    // Offset of CP
    let Off_CP = restore_Off_CP();

    // Kv CP
    let K_V_CP = restore_K_V_CP();

    // Kta CP
    let K_Ta_CP = restore_K_Ta_CP();

    // TGC
    let TGC = restore_TGC();

    // Resolution control
    let Resolution = restore_Resolution();

    let pattern = restore_pattern();

    return EepromVars {
        K_Vdd,
        VDD_25,

        K_V_PTAT,
        K_T_PTAT,
        V_PTAT_25,
        Alpha_PTAT,

        pix_os_ref,

        a,

        K_V,

        K_Ta,

        GAIN,

        Ks_Ta,

        Step,
        CT3,
        CT4,

        Ks_To,

        Alpha_corr,

        a_CP,

        Off_CP,

        K_V_CP,

        K_Ta_CP,

        TGC,

        Resolution,

        pattern,
    };
}

// -------------------------------------
// | Temperature calculation functions |
// -------------------------------------

fn calc_V_dd(Resolution_corr: f32) -> f32 {
    let VDD_25: f32 = EEPROM_VARS.VDD_25 as f32;
    let K_Vdd: f32 = EEPROM_VARS.K_Vdd as f32;

    let mut V_ram: f32 = super::read_value(0x072A) as f32;
    if V_ram > 32767.0 { V_ram -= 65536.0 }
    let V_dd: f32 = (Resolution_corr * V_ram - VDD_25) / K_Vdd + 3.3;
    return V_dd;
}

fn calc_T_a() -> f32 {
    let VDD_25 = EEPROM_VARS.VDD_25;
    let K_Vdd = EEPROM_VARS.K_Vdd;
    let K_V_PTAT = EEPROM_VARS.K_V_PTAT;
    let K_T_PTAT = EEPROM_VARS.K_T_PTAT;
    let V_PTAT_25 = EEPROM_VARS.V_PTAT_25;
    let Alpha_PTAT = EEPROM_VARS.Alpha_PTAT;

    let mut dV: f32 = super::read_value(0x072A) as f32;
    if dV > 32767.0 {
        dV -= 65536.0;
    }
    dV -= VDD_25 as f32;
    dV /= K_Vdd as f32;

    let mut V_PTAT: f32 = super::read_value(0x0720) as f32;
    if V_PTAT > 32767.0 {
        V_PTAT -= 65536.0;
    }

    let mut V_BE: f32 = super::read_value(0x0700) as f32;
    if V_BE > 32767.0 {
        V_BE -= 65536.0;
    }

    let V_PTAT_art: f32 = (V_PTAT as f32 / (V_PTAT * Alpha_PTAT + V_BE)) * 2_f32.powi(18);

    let mut T_a: f32 = V_PTAT_art / (1.0 + K_V_PTAT * dV);
    T_a -= V_PTAT_25 as f32;
    T_a /= K_T_PTAT;
    T_a += 25.0;

    return T_a;
}

fn calc_K_gain() -> f32 {
    let GAIN: f32 = EEPROM_VARS.GAIN as f32;

    let mut gain_ram: f32 = super::read_value(0x070A) as f32;
    if gain_ram > 32767.0 { gain_ram -= 65536.0 }
    let K_gain: f32 = GAIN / gain_ram;
    return K_gain;
}

fn calc_pix_gain(K_gain: f32, pixel_data: [u16; PIXEL_COUNT]) -> [f32; PIXEL_COUNT] {
    let mut pix_gain: [f32; PIXEL_COUNT] = [0.0; PIXEL_COUNT];
    for i in 0..PIXEL_COUNT {
        let mut p: f32 = pixel_data[i] as f32;
        if p > 32767.0 { p -= 65536.0 }
        pix_gain[i] = p * K_gain;
    }
    return pix_gain;
}

fn calc_pix_os(pix_gain: [f32; PIXEL_COUNT], V_dd: f32, T_a: f32) -> [f32; PIXEL_COUNT] {
    let K_Ta = EEPROM_VARS.K_Ta;
    let K_V = EEPROM_VARS.K_V;
    let pix_os_ref = EEPROM_VARS.pix_os_ref;

    let mut pix_os: [f32; PIXEL_COUNT] = [0.0; PIXEL_COUNT];
    for i in 0..PIXEL_COUNT {
        pix_os[i] = pix_gain[i];

        let coef_1: f32 = 1.0 + K_Ta[i] * (T_a - 25.0);
        let coef_2: f32 = 1.0 + K_V[i] * (V_dd - 3.3);

        pix_os[i] -= pix_os_ref[i] as f32 * coef_1 * coef_2;
    }
    return pix_os;
}

fn calc_V_IR_Em_compensated(pix_os: [f32; PIXEL_COUNT], emissivity: f32) -> [f32; PIXEL_COUNT] {
    let mut V_IR_Em_compensated: [f32; PIXEL_COUNT] = [0.0; PIXEL_COUNT];
    for i in 0..PIXEL_COUNT {
        V_IR_Em_compensated[i] = pix_os[i] / emissivity;
    }
    return V_IR_Em_compensated;
}

fn calc_pix_OS_CP_SPX(V_dd: f32, T_a: f32, K_gain: f32) -> (f32, f32) {
    let K_Ta_CP = EEPROM_VARS.K_Ta_CP;
    let K_V_CP = EEPROM_VARS.K_V_CP;
    let Off_CP = EEPROM_VARS.Off_CP;

    let mut pix_gain_CP_SP0_RAM = super::read_value(0x0708) as f32;
    let mut pix_gain_CP_SP1_RAM = super::read_value(0x0728) as f32;
    if pix_gain_CP_SP0_RAM > 32767.0 { pix_gain_CP_SP0_RAM -= 65536.0 }
    if pix_gain_CP_SP1_RAM > 32767.0 { pix_gain_CP_SP1_RAM -= 65536.0 }

    let pix_gain_CP_SP0 = pix_gain_CP_SP0_RAM * K_gain;
    let pix_gain_CP_SP1 = pix_gain_CP_SP1_RAM * K_gain;

    let coef_1 = 1.0 + K_Ta_CP * (T_a - 25.0);
    let coef_2 = 1.0 + K_V_CP * (V_dd - 3.3);

    let mut pix_OS_CP_SP0 = pix_gain_CP_SP0;
    let mut pix_OS_CP_SP1 = pix_gain_CP_SP1;

    pix_OS_CP_SP0 -= Off_CP.0 as f32 * coef_1 * coef_2;
    pix_OS_CP_SP1 -= Off_CP.1 as f32 * coef_1 * coef_2;

    return (pix_OS_CP_SP0, pix_OS_CP_SP1);
}

fn calc_V_IR_compensated(V_IR_Em_compensated: [f32; PIXEL_COUNT], pix_OS_CP_SP: (f32, f32)) -> [f32; PIXEL_COUNT] {
    let TGC = EEPROM_VARS.TGC;
    let pattern = EEPROM_VARS.pattern;

    let mut V_IR_compensated: [f32; PIXEL_COUNT] = [0.0; PIXEL_COUNT];
    for i in 0..PIXEL_COUNT {
        V_IR_compensated[i] = V_IR_Em_compensated[i];
        V_IR_compensated[i] -= TGC * ((1 - pattern[i]) as f32 * pix_OS_CP_SP.0 + pattern[i] as f32 * pix_OS_CP_SP.1);
    }
    return V_IR_compensated;
}

fn calc_a_comp(T_a: f32) -> [f32; PIXEL_COUNT] {
    let a = EEPROM_VARS.a;
    let TGC = EEPROM_VARS.TGC;
    let pattern = EEPROM_VARS.pattern;
    let a_CP = EEPROM_VARS.a_CP;
    let Ks_Ta = EEPROM_VARS.Ks_Ta;

    let mut a_comp: [f32; PIXEL_COUNT] = [0.0; PIXEL_COUNT];
    for i in 0..PIXEL_COUNT {
        a_comp[i] = a[i];

        a_comp[i] -= TGC * ((1 - pattern[i]) as f32 * a_CP.0 + pattern[i] as f32 * a_CP.1);

        a_comp[i] *= 1.0 + Ks_Ta * (T_a - 25.0);
    }

    return a_comp;
}

fn calc_T_o(emissivity: f32, T_a: f32, V_IR_compensated: [f32; PIXEL_COUNT], a_comp: [f32; PIXEL_COUNT]) -> [f32; PIXEL_COUNT] {
    let Ks_To2 = EEPROM_VARS.Ks_To.2;

    let T_r = T_a - 8.0;
    let T_aK4 = (T_a + 273.15).powi(4);
    let T_rK4 = (T_r + 273.15).powi(4);

    let T_a_r = T_rK4 - (T_rK4 - T_aK4) / emissivity;

    let mut S_x: [f32; PIXEL_COUNT] = [0.0; PIXEL_COUNT];
    for i in 0..PIXEL_COUNT {
        S_x[i] = Ks_To2;

        S_x[i] *= (a_comp[i].powi(3) * V_IR_compensated[i] + a_comp[i].powi(4) * T_a_r).powf(1.0 / 4.0);
    }

    let mut T_o: [f32; PIXEL_COUNT] = [0.0; PIXEL_COUNT];
    for i in 0..PIXEL_COUNT {
        T_o[i] = V_IR_compensated[i];
        T_o[i] /= a_comp[i] * (1.0 - Ks_To2 * 273.15) + S_x[i];
        T_o[i] += T_a_r;
        T_o[i] = T_o[i].powf(1.0 / 4.0);
        T_o[i] -= 273.15;
    }

    return T_o;
}

// ----------------------------
// | EEPROM restore functions |
// ----------------------------

fn restore_K_Vdd() -> i32 {
    let mut K_Vdd: i32 = ((get_eeprom_val(0x2433) & 0xFF00) >> 8) as i32;
    if K_Vdd > 127 {
        K_Vdd -= 256;
    }
    K_Vdd *= 2_i32.pow(5);
    return K_Vdd;
}

fn restore_VDD_25() -> i32 {
    let mut VDD_25: i32 = (get_eeprom_val(0x2433) & 0x00FF) as i32;
    VDD_25 = ((VDD_25 - 256) * 2_i32.pow(5)) - 2_i32.pow(13);
    return VDD_25;
}

fn restore_T_a(K_V_PTAT: &mut f32, K_T_PTAT: &mut f32, V_PTAT_25: &mut i32, Alpha_PTAT: &mut f32) {
    *K_V_PTAT = ((get_eeprom_val(0x2432) & 0xFC00) >> 10) as f32;
    if *K_V_PTAT > 31.0 {
        *K_V_PTAT -= 64.0;
    }
    *K_V_PTAT /= 2_f32.powi(12);

    *K_T_PTAT = (get_eeprom_val(0x2432) & 0x03FF) as f32;
    if *K_T_PTAT > 511.0 {
        *K_T_PTAT -= 1024.0;
    }
    *K_T_PTAT /= 2_f32.powi(3);

    *V_PTAT_25 = get_eeprom_val(0x2431) as i32;
    if *V_PTAT_25 > 32767 {
        *V_PTAT_25 -= 65536;
    }

    let Alpha_PTAT_EE: f32 = ((get_eeprom_val(0x2410) & 0xF000) >> 12) as f32;
    *Alpha_PTAT = Alpha_PTAT_EE / 2_f32.powi(2) + 8.0;
}

fn restore_offset() -> [i32; PIXEL_COUNT] {
    let mut offset_avg: i32 = get_eeprom_val(0x2411) as i32;
    if offset_avg > 32767 {
        offset_avg -= 65536;
    }

    // OCC row i
    let mut OCC_row: [i32; PIXELS_HEIGHT] = [0x00; PIXELS_HEIGHT];
    for row in 0..PIXELS_HEIGHT/4 {
        let address: u16 = (0x2412 + row) as u16;

        OCC_row[row * 4 + 0] = ((get_eeprom_val(address) & 0x000F) >> 0) as i32;
        OCC_row[row * 4 + 1] = ((get_eeprom_val(address) & 0x00F0) >> 4) as i32;
        OCC_row[row * 4 + 2] = ((get_eeprom_val(address) & 0x0F00) >> 8) as i32;
        OCC_row[row * 4 + 3] = ((get_eeprom_val(address) & 0xF000) >> 12) as i32;

        if OCC_row[row * 4 + 0] > 7 { OCC_row[row * 4 + 0] -= 16 }
        if OCC_row[row * 4 + 1] > 7 { OCC_row[row * 4 + 1] -= 16 }
        if OCC_row[row * 4 + 2] > 7 { OCC_row[row * 4 + 2] -= 16 }
        if OCC_row[row * 4 + 3] > 7 { OCC_row[row * 4 + 3] -= 16 }
    }

    // OCC scale row
    let OCC_scale_row: u16 = (get_eeprom_val(0x2410) & 0x0F00) as u16 >> 8;

    // OCC column
    let mut OCC_column: [i32; PIXELS_WIDTH] = [0x00; PIXELS_WIDTH];
    for column in 0..PIXELS_WIDTH/4 {
        let address: u16 = (0x2418 + column) as u16;

        OCC_column[column * 4 + 0] = ((get_eeprom_val(address) & 0x000F) >> 0) as i32;
        OCC_column[column * 4 + 1] = ((get_eeprom_val(address) & 0x00F0) >> 4) as i32;
        OCC_column[column * 4 + 2] = ((get_eeprom_val(address) & 0x0F00) >> 8) as i32;
        OCC_column[column * 4 + 3] = ((get_eeprom_val(address) & 0xF000) >> 12) as i32;

        if OCC_column[column * 4 + 0] > 7 { OCC_column[column * 4 + 0] -= 16 }
        if OCC_column[column * 4 + 1] > 7 { OCC_column[column * 4 + 1] -= 16 }
        if OCC_column[column * 4 + 2] > 7 { OCC_column[column * 4 + 2] -= 16 }
        if OCC_column[column * 4 + 3] > 7 { OCC_column[column * 4 + 3] -= 16 }
    }

    // OCC scale column
    let OCC_scale_column: u16 = (get_eeprom_val(0x2410) & 0x00F0) as u16 >> 4;

    // offset
    let mut offset: [i32; PIXEL_COUNT] = [0x00; PIXEL_COUNT];
    for i in 0..PIXEL_COUNT {
        let address: u16 = (0x2440 + i) as u16;

        offset[i] = ((get_eeprom_val(address) & 0xFC00) >> 10) as i32;
        if offset[i] > 31 {
            offset[i] -= 64;
        }
    }

    // OCC scale remnant
    let OCC_scale_remnant: u16 = get_eeprom_val(0x2410) as u16 & 0x000F;

    let mut pix_os_ref: [i32; PIXEL_COUNT] = [0x00; PIXEL_COUNT];
    for i in 0..PIXELS_HEIGHT {
        for j in 0..PIXELS_WIDTH {
            let index = i * PIXELS_WIDTH + j;
            pix_os_ref[index] = offset_avg;
            pix_os_ref[index] += OCC_row[i] << OCC_scale_row;
            pix_os_ref[index] += OCC_column[j] << OCC_scale_column;
            pix_os_ref[index] += offset[index] << OCC_scale_remnant;
        }
    }
    return pix_os_ref;
}

fn restore_a() -> [f32; PIXEL_COUNT] {
    let a_reference: i32 = get_eeprom_val(0x2421) as i32;

    let a_scale: i32 = ((get_eeprom_val(0x2420) & 0xF000) >> 12) as i32 + 30;

    let mut ACC_row: [i32; PIXELS_HEIGHT] = [0x00; PIXELS_HEIGHT];
    for row in 0..PIXELS_HEIGHT/4 {
        let address: u16 = 0x2422 + row as u16;

        ACC_row[row * 4 + 0] = ((get_eeprom_val(address) & 0x000F) >> 0) as i32;
        ACC_row[row * 4 + 1] = ((get_eeprom_val(address) & 0x00F0) >> 4) as i32;
        ACC_row[row * 4 + 2] = ((get_eeprom_val(address) & 0x0F00) >> 8) as i32;
        ACC_row[row * 4 + 3] = ((get_eeprom_val(address) & 0xF000) >> 12) as i32;

        if ACC_row[row * 4 + 0] > 7 { ACC_row[row * 4 + 0] -= 16 }
        if ACC_row[row * 4 + 1] > 7 { ACC_row[row * 4 + 1] -= 16 }
        if ACC_row[row * 4 + 2] > 7 { ACC_row[row * 4 + 2] -= 16 }
        if ACC_row[row * 4 + 3] > 7 { ACC_row[row * 4 + 3] -= 16 }
    }

    let ACC_scale_row: u16 = (get_eeprom_val(0x2420) & 0x0F00) as u16 >> 8;

    let mut ACC_column: [i32; PIXELS_WIDTH] = [0x00; PIXELS_WIDTH];
    for column in 0..PIXELS_WIDTH/4 {
        let address: u16 = 0x2428 + column as u16;

        ACC_column[column * 4 + 0] = ((get_eeprom_val(address) & 0x000F) >> 0) as i32;
        ACC_column[column * 4 + 1] = ((get_eeprom_val(address) & 0x00F0) >> 4) as i32;
        ACC_column[column * 4 + 2] = ((get_eeprom_val(address) & 0x0F00) >> 8) as i32;
        ACC_column[column * 4 + 3] = ((get_eeprom_val(address) & 0xF000) >> 12) as i32;

        if ACC_column[column * 4 + 0] > 7 { ACC_column[column * 4 + 0] -= 16 }
        if ACC_column[column * 4 + 1] > 7 { ACC_column[column * 4 + 1] -= 16 }
        if ACC_column[column * 4 + 2] > 7 { ACC_column[column * 4 + 2] -= 16 }
        if ACC_column[column * 4 + 3] > 7 { ACC_column[column * 4 + 3] -= 16 }
    }

    let ACC_scale_column: u16 = (get_eeprom_val(0x2420) & 0x00F0) as u16 >> 4;

    let mut a_pixel: [i32; PIXEL_COUNT] = [0x00; PIXEL_COUNT];
    for i in 0..PIXEL_COUNT {
        let address: u16 = 0x2440 + i as u16;

        a_pixel[i] = ((get_eeprom_val(address) & 0x03F0) >> 4) as i32;
        if a_pixel[i] > 31 {
            a_pixel[i] -= 64;
        }
    }

    let ACC_scale_remnant: u16 = get_eeprom_val(0x2420) as u16 & 0x000F;

    let mut a: [f32; PIXEL_COUNT] = [0.0; PIXEL_COUNT];
    for i in 0..PIXELS_HEIGHT {
        for j in 0..PIXELS_WIDTH {
            let index = i * PIXELS_WIDTH + j;

            a[index] = a_reference as f32;
            a[index] += ACC_row[i] as f32 * 2_f32.powi(ACC_scale_row as i32);
            a[index] += ACC_column[j] as f32 * 2_f32.powi(ACC_scale_column as i32);
            a[index] += a_pixel[index] as f32 * 2_f32.powi(ACC_scale_remnant as i32);
            a[index] /= 2_f32.powi(a_scale as i32);
        }
    }
    return a;
}

fn restore_K_V() -> [f32; PIXEL_COUNT] {
    let K_V_scale: u16 = (get_eeprom_val(0x2438) & 0x0F00) as u16 >> 8;

    let mut K_V: [f32; PIXEL_COUNT] = [0.0; PIXEL_COUNT];
    // EVEN EVEN
    for i in (0..PIXELS_HEIGHT).step_by(2) {
        for j in (0..PIXELS_WIDTH).step_by(2) {
            let index = i * PIXELS_WIDTH + j;
            K_V[index] = ((get_eeprom_val(0x2434) & 0xF000) >> 12) as i32 as f32;
        }
    }

    // ODD EVEN
    for i in (1..PIXELS_HEIGHT).step_by(2) {
        for j in (0..PIXELS_WIDTH).step_by(2) {
            let index = i * PIXELS_WIDTH + j;
            K_V[index] = ((get_eeprom_val(0x2434) & 0x0F00) >> 8) as i32 as f32;
        }
    }

    // EVEN ODD
    for i in (0..PIXELS_HEIGHT).step_by(2) {
        for j in (1..PIXELS_WIDTH).step_by(2) {
            let index = i * PIXELS_WIDTH + j;
            K_V[index] = ((get_eeprom_val(0x2434) & 0x00F0) >> 4) as i32 as f32;
        }
    }

    // ODD ODD
    for i in (1..PIXELS_HEIGHT).step_by(2) {
        for j in (1..PIXELS_WIDTH).step_by(2) {
            let index = i * PIXELS_WIDTH + j;
            K_V[index] = ((get_eeprom_val(0x2434) & 0x000F) >> 0) as i32 as f32;
        }
    }

    for i in 0..PIXEL_COUNT {
        if K_V[i] > 7.0 {
            K_V[i] -= 16.0;
        }

        K_V[i] /= 2_f32.powi(K_V_scale as i32);
    }

    return K_V;
}

fn restore_K_Ta() -> [f32; PIXEL_COUNT] {
    let mut K_Ta_EE: [i32; PIXEL_COUNT] = [0x00; PIXEL_COUNT];

    for i in 0..PIXEL_COUNT {
        let address: u16 = 0x2440 + i as u16;

        K_Ta_EE[i] = ((get_eeprom_val(address) & 0x000E) >> 1) as i32;
        if K_Ta_EE[i] > 3 {
            K_Ta_EE[i] -= 8;
        }
    }

    let mut K_Ta_RC_EE: [i32; PIXEL_COUNT] = [0x00; PIXEL_COUNT];

    // EVEN EVEN
    for i in (0..PIXELS_HEIGHT).step_by(2) {
        for j in (0..PIXELS_WIDTH).step_by(2) {
            let index = i * PIXELS_WIDTH + j;
            K_Ta_RC_EE[index] = ((get_eeprom_val(0x2436) & 0xFF00) >> 8) as i32;
        }
    }

    // ODD EVEN
    for i in (1..PIXELS_HEIGHT).step_by(2) {
        for j in (0..PIXELS_WIDTH).step_by(2) {
            let index = i * PIXELS_WIDTH + j;
            K_Ta_RC_EE[index] = ((get_eeprom_val(0x2436) & 0x00FF) >> 0) as i32;
        }
    }

    // EVEN ODD
    for i in (0..PIXELS_HEIGHT).step_by(2) {
        for j in (1..PIXELS_WIDTH).step_by(2) {
            let index = i * PIXELS_WIDTH + j;
            K_Ta_RC_EE[index] = ((get_eeprom_val(0x2437) & 0xFF00) >> 8) as i32;
        }
    }

    // ODD ODD
    for i in (1..PIXELS_HEIGHT).step_by(2) {
        for j in (1..PIXELS_WIDTH).step_by(2) {
            let index = i * PIXELS_WIDTH + j;
            K_Ta_RC_EE[index] = ((get_eeprom_val(0x2437) & 0x00FF) >> 0) as i32;
        }
    }

    for i in 0..PIXEL_COUNT {
        if K_Ta_RC_EE[i] > 127 {
            K_Ta_RC_EE[i] -= 256;
        }
    }

    let K_Ta_scale1: u16 = ((get_eeprom_val(0x2438) & 0x00F0) as u16 >> 4) + 8;

    let K_Ta_scale2: u16 = (get_eeprom_val(0x2438) & 0x000F) as u16;

    let mut K_Ta: [f32; PIXEL_COUNT] = [0.0; PIXEL_COUNT];

    for i in 0..PIXEL_COUNT {
        K_Ta[i] = K_Ta_RC_EE[i] as f32;
        K_Ta[i] += K_Ta_EE[i] as f32 * 2_f32.powi(K_Ta_scale2 as i32);
        K_Ta[i] /= 2_f32.powi(K_Ta_scale1 as i32);
    }

    return K_Ta;
}

fn restore_gain() -> i32 {
    let mut gain: i32 = get_eeprom_val(0x2430) as i32;
    if gain > 32767 {
        gain -= 65536;
    }
    return gain;
}

fn restore_Ks_Ta() -> f32 {
    let mut Ks_Ta_EE: i32 = ((get_eeprom_val(0x243C) & 0xFF00) >> 8) as i32;
    if Ks_Ta_EE > 127 {
        Ks_Ta_EE -= 256;
    }

    let Ks_Ta: f32 = Ks_Ta_EE as f32 / 2_f32.powi(13);
    return Ks_Ta;
}

fn restore_Step() -> i32 {
    return ((get_eeprom_val(0x243F) & 0x3000) >> 12) as i32 * 10;
}

fn restore_CT3(Step: i32) -> i32 {
    return ((get_eeprom_val(0x243F) & 0x00F0) >> 4) as i32 * Step;
}

fn restore_CT4(Step: i32, CT3: i32) -> i32 {
    return ((get_eeprom_val(0x243F) & 0x0F00) >> 8) as i32 * Step + CT3;
}

fn restore_Ks_To() -> (f32, f32, f32, f32) {
    let Ks_To_scale: u16 = (get_eeprom_val(0x243F) & 0x000F) + 8;

    let mut Ks_To1_EE: i32 = (get_eeprom_val(0x243D) & 0x00FF) as i32;
    if Ks_To1_EE > 127 { Ks_To1_EE -= 256 }
    let Ks_To1 = Ks_To1_EE as f32 / 2_f32.powi(Ks_To_scale as i32);

    let mut Ks_To2_EE: i32 = ((get_eeprom_val(0x243D) & 0xFF00) >> 8) as i32;
    if Ks_To2_EE > 127 { Ks_To2_EE -= 256 }
    let Ks_To2 = Ks_To2_EE as f32 / 2_f32.powi(Ks_To_scale as i32);

    let mut Ks_To3_EE: i32 = (get_eeprom_val(0x243E) & 0x00FF) as i32;
    if Ks_To3_EE > 127 { Ks_To3_EE -= 256 }
    let Ks_To3 = Ks_To3_EE as f32 / 2_f32.powi(Ks_To_scale as i32);

    let mut Ks_To4_EE: i32 = ((get_eeprom_val(0x243E) & 0xFF00) >> 8) as i32;
    if Ks_To4_EE > 127 { Ks_To4_EE -= 256 }
    let Ks_To4 = Ks_To4_EE as f32 / 2_f32.powi(Ks_To_scale as i32);

    return (Ks_To1, Ks_To2, Ks_To3, Ks_To4);
}

fn restore_Alpha_corr(Ks_To: (f32, f32, f32, f32), CT3: i32, CT4: i32) -> (f32, f32, f32, f32) {
    let Alpha_corr_range1: f32 = 1.0 / (1.0 + Ks_To.1 * 40.0);
    let Alpha_corr_range2: f32 = 1.0;
    let Alpha_corr_range3: f32 = 1.0 + Ks_To.2 * CT3 as f32;
    let Alpha_corr_range4: f32 = Alpha_corr_range3 * (1.0 + Ks_To.2 * (CT4 - CT3) as f32);

    return (Alpha_corr_range1, Alpha_corr_range2, Alpha_corr_range3, Alpha_corr_range4);
}

fn restore_a_CP() -> (f32, f32) {
    let a_scale_CP = ((get_eeprom_val(0x2420) & 0xF000) >> 12) as i32 + 27;
    let mut CP_P1_P0_ratio = ((get_eeprom_val(0x2439) & 0xFC00) >> 10) as i32;
    if CP_P1_P0_ratio > 31 {
        CP_P1_P0_ratio -= 64;
    }

    let a_CP_0 = ((get_eeprom_val(0x2439) & 0x03FF)) as i32 as f32 / 2_f32.powi(a_scale_CP as i32);
    let a_CP_1 = a_CP_0 * (1.0 + (CP_P1_P0_ratio as f32 / 2_f32.powi(7)));

    return (a_CP_0, a_CP_1);
}

fn restore_Off_CP() -> (i32, i32) {
    let mut Off_CP_0 = (get_eeprom_val(0x243A) & 0x03FF) as i32;
    if Off_CP_0 > 511 {
        Off_CP_0 -= 1024;
    }

    let mut Off_CP_1_delta: i32 = ((get_eeprom_val(0x243A) & 0xFC00) >> 10) as i32;
    if Off_CP_1_delta > 31 {
        Off_CP_1_delta -= 64;
    }

    let Off_CP_1 = Off_CP_0 + Off_CP_1_delta;

    return (Off_CP_0, Off_CP_1);
}

fn restore_K_V_CP() -> f32 {
    let K_V_Scale: u16 = (get_eeprom_val(0x2438) & 0x0F00) as u16 >> 8;

    let mut K_V_CP_EE: i32 = ((get_eeprom_val(0x243B) & 0xFF00) >> 8) as i32;
    if K_V_CP_EE > 127 {
        K_V_CP_EE -= 256;
    }

    let K_V_CP: f32 = K_V_CP_EE as f32 / 2_f32.powi(K_V_Scale as i32);
    return K_V_CP;
}

fn restore_K_Ta_CP() -> f32 {
    let K_Ta_scale_1: u16 = ((get_eeprom_val(0x2438) & 0x00F0) as u16 >> 4) + 8;

    let mut K_Ta_CP_EE: i32 = (get_eeprom_val(0x243B) & 0x00FF) as i32;
    if K_Ta_CP_EE > 127 {
        K_Ta_CP_EE -= 256;
    }

    let K_Ta_CP = K_Ta_CP_EE as f32 / 2_f32.powi(K_Ta_scale_1 as i32);
    return K_Ta_CP;
}

fn restore_TGC() -> f32 {
    let mut TGC_EE: i32 = (get_eeprom_val(0x243C) & 0x00FF) as i32;
    if TGC_EE > 127 {
        TGC_EE -= 256;
    }

    let TGC = TGC_EE as f32 / 2_f32.powi(5);
    return TGC;
}

fn restore_Resolution() -> u16 {
    return (get_eeprom_val(0x2438) & 0x3000) as u16 >> 12;
}

fn restore_pattern() -> [u16; PIXEL_COUNT] {
    let mut pattern: [u16; PIXEL_COUNT] = [0x00; PIXEL_COUNT];

    for i in 0..PIXEL_COUNT {
        pattern[i] = i as u16 / 32;
        pattern[i] -= pattern[i] & !0x0001;

        let mut v = i as u16;
        v -= i as u16 & !0x0001;

        pattern[i] = pattern[i] ^ v;
    }

    return pattern;
}
