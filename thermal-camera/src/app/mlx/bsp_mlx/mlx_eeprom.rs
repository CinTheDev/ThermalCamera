#![allow(non_snake_case)]

use lazy_static::lazy_static;

const PIXELS_WIDTH: usize = 32;
const PIXELS_HEIGHT: usize = 24;
const PIXEL_COUNT: usize = PIXELS_WIDTH * PIXELS_HEIGHT;

const EEPROM_SIZE: usize = 767;

pub struct EepromVars {
    K_Vdd: i16,
    VDD_25: i16,

    T_a: f32,

    pix_os_ref: [i16; PIXEL_COUNT],

    a: [i16; PIXEL_COUNT],

    K_V: [f32; PIXEL_COUNT],

    K_Ta: [f32; PIXEL_COUNT],

    GAIN: i16,

    Ks_Ta: i16,

    Step: i16,
    CT3: i16,
    CT4: i16,

    Ks_To1: i16,
    Ks_To2: i16,
    Ks_To3: i16,
    Ks_To4: i16,

    Alpha_corr_1: f32,
    Alpha_corr_2: f32,
    Alpha_corr_3: f32,
    Alpha_corr_4: f32,

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

pub fn evaluate(pix_data: [u16; PIXEL_COUNT]) -> f32 {
    // We keep Resolution at default, so the coefficient will be just 1
    let Resolution_corr = 1;

    // Calculate Voltage
    let V_dd:f32 = (Resolution_corr * super::read_value(0x072A) as i16 - EEPROM_VARS.VDD_25) as f32 / (EEPROM_VARS.K_Vdd as f32 + 3.3);

    // Calculate Ambient temperature
    let T_a: f32 = EEPROM_VARS.T_a;

    // Compensate for gain
    let K_gain:f32 = EEPROM_VARS.GAIN as f32 / (super::read_value(0x070A) as i16) as f32;

    let pix_gain: [f32; PIXEL_COUNT];
    for i in 0..PIXEL_COUNT {
        pix_gain[i] = pix_data[i] as f32 * K_gain;
    }

    // Offset, VDD and Ta
    let pix_os: [f32; PIXEL_COUNT];
    for i in 0..PIXEL_COUNT {
        pix_os[i] = pix_gain[i];

        let coef_1: f32 = 1.0 + EEPROM_VARS.K_Ta[i] * (T_a - 25.0);
        let coef_2: f32 = 1.0 + EEPROM_VARS.K_V[i] * (V_dd - 3.3);

        pix_os[i] -= EEPROM_VARS.pix_os_ref[i] * coef_1 * coef_2;
    }

    // Emissivity compensation
    // In example the result is divided by 1, so I'm leaving this step out
    // TODO: Maybe add emissivity in the future
    let mut V_IR_Em_compensated: [f32; PIXEL_COUNT];
    for i in 0..PIXEL_COUNT {
        V_IR_Em_compensated[i] = pix_os[i] / 1.0;
    }

    // CP gain compensation
    let pix_gain_CP_SP0: f32 = super::read_value(0x0708) as i16 * K_gain;
    let pix_gain_CP_SP1: f32 = super::read_value(0x0728) as i16 * K_gain;

    let mut pix_OS_CP_SP0: [f32; PIXEL_COUNT];
    let mut pix_OS_CP_SP1: [f32; PIXEL_COUNT];
    for i in 0..PIXEL_COUNT {
        pix_OS_CP_SP0[i] = pix_gain_CP_SP0;
        pix_OS_CP_SP1[i] = pix_gain_CP_SP1;

        let coef_1: f32 = (1 + EEPROM_VARS.K_Ta_CP * (EEPROM_VARS.T_a - 25.0));
        let coef_2: f32 = (1 + EEPROM_VARS.K_V_CP * (V_dd - 3.3));

        pix_OS_CP_SP0[i] -= EEPROM_VARS.Off_CP_0 * coef_1 * coef_2;
        pix_OS_CP_SP1[i] -= EEPROM_VARS.Off_CP_1 * coef_1 * coef_2;
    }

    // Gradient compensation
    let mut pattern: [u16; PIXEL_COUNT];

    for i in 0..PIXEL_COUNT {
        pattern[i] = (i - 1) >> 5;
        pattern[i] -= pattern[i] & !0x0001;

        let mut v: u16 = i - 1;
        v -= (i - 1) & !0x0001;

        pattern[i] = pattern[i] ^ v;
    }

    let V_IR_compensated: [f32; PIXEL_COUNT];
    for i in 0..PIXEL_COUNT {
        V_IR_compensated[i] = V_IR_Em_compensated[i];

        V_IR_compensated[i] -= EEPROM_VARS.TGC * ((1 - pattern[i]) * pix_OS_CP_SP0[i] + pattern[i] * pix_OS_CP_SP1[i]);
    }

    // Normalize to sensitivity
    let mut a_comp: [f32; PIXEL_COUNT];
    for i in 0..PIXEL_COUNT {
        a_comp[i] = EEPROM_VARS.a[i];

        a_comp[i] -= EEPROM_VARS.TGC * ((1 - pattern[i] * EEPROM_VARS.a_CP_0 + pattern * EEPROM_VARS.a_CP_1));

        a_comp[i] *= 1 + EEPROM_VARS.Ks_Ta * (EEPROM_VARS.T_a - 25.0);
    }

    // Calculate To
    let T_r = T_a - 8.0;
    let T_aK4 = (T_a + 273.15).powi(4);
    let T_rK4 = (T_r + 273.15).powi(4);

    let T_a_r = T_rK4 - (T_rK4 - T_aK4) / 1.0; // 1.0 is emissivity

    let S_x: [f32; PIXEL_COUNT];
    for i in 0..PIXEL_COUNT {
        S_x[i] = EEPROM_VARS.Ks_To2;

        // This is fourth root
        S_x *= (a_comp[i].powi(3) * V_IR_compensated[i] + a_comp[i].powi(4) * T_a_r).powf(1.0 / 4.0);
    }

    let T_o: [f32; PIXEL_COUNT];
    for i in 0..PIXEL_COUNT {
        T_o[i] = V_IR_compensated[i];
        T_o[i] /= a_comp[i] * (1 - EEPROM_VARS.Ks_To2 * 273.15) + S_x[i];
        T_o[i] += T_a_r;
        T_o[i] = T_o[i].powf(1.0 / 4.0);
        T_o[i] -= 273.15;
    }

    // TODO: do additional temperature ranges
    return T_o;
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
    };
}

fn calc_K_Vdd() -> i16 {
    let mut K_Vdd: i16 = ((get_eeprom_val(0x2433) & 0xFF00) >> 8) as i16;
    if K_Vdd > 127 {
        K_Vdd -= 256;
    }
    K_Vdd <<= 5;
    return K_Vdd;
}

fn calc_VDD_25() -> i16 {
    let mut VDD_25: i16 = (get_eeprom_val(0x2433) & 0x00FF) as i16;
    VDD_25 = ((VDD_25 - 256) << 5) - (2 as i16).pow(13);
    return VDD_25;
}

fn calc_T_a(VDD_25: i16) -> f32 {
    let mut K_V_PTAT: f32 = ((get_eeprom_val(0x2432) & 0xFC00) >> 10) as f32;
    if K_V_PTAT > 31.0 {
        K_V_PTAT -= 64.0;
    }
    K_V_PTAT /= (2 as f32).powi(12);

    let mut K_T_PTAT: f32 = (get_eeprom_val(0x2432) & 0x3FF) as f32;
    if K_T_PTAT > 511.0 {
        K_T_PTAT -= 1024.0;
    }
    K_T_PTAT /= (2.0 as f32).powi(3);

    let dV: f32 = (super::read_value(0x072A) as i16 - VDD_25) as f32 / K_V_PTAT; // Datasheet just says K_V, i guessed it to be K_V_PTAT

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

    let V_PTAT_art: f32 = (V_PTAT as f32 / (V_PTAT * Alpha_PTAT + V_BE) as f32) * (2.0 as f32).powi(18);

    let mut T_a: f32 = V_PTAT_art / (1.0 + K_V_PTAT * dV);
    T_a -= V_PTAT_25 as f32;
    T_a /= K_T_PTAT;
    T_a += 25.0;

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

fn calc_K_V() -> [f32; PIXEL_COUNT] {
    let K_V_scale: u16 = (get_eeprom_val(0x2438) & 0x0F00) as u16 >> 8;

    let mut K_V: [f32; PIXEL_COUNT] = [0.0; PIXEL_COUNT];
    // EVEN EVEN
    for i in (0..PIXELS_HEIGHT).step_by(2) {
        for j in (0..PIXELS_WIDTH).step_by(2) {
            let index = i * PIXELS_WIDTH + j;
            K_V[index] = ((get_eeprom_val(0x2434) & 0xF000) >> 12) as i16 as f32;
        }
    }

    // ODD EVEN
    for i in (1..PIXELS_HEIGHT).step_by(2) {
        for j in (0..PIXELS_WIDTH).step_by(2) {
            let index = i * PIXELS_WIDTH + j;
            K_V[index] = ((get_eeprom_val(0x2434) & 0x0F00) >> 8) as i16 as f32;
        }
    }

    // EVEN ODD
    for i in (0..PIXELS_HEIGHT).step_by(2) {
        for j in (1..PIXELS_WIDTH).step_by(2) {
            let index = i * PIXELS_WIDTH + j;
            K_V[index] = ((get_eeprom_val(0x2434) & 0x00F0) >> 4) as i16 as f32;
        }
    }

    // ODD ODD
    for i in (1..PIXELS_HEIGHT).step_by(2) {
        for j in (1..PIXELS_WIDTH).step_by(2) {
            let index = i * PIXELS_WIDTH + j;
            K_V[index] = ((get_eeprom_val(0x2434) & 0x000F) >> 0) as i16 as f32;
        }
    }

    for i in 0..PIXEL_COUNT {
        if K_V[i] > 7.0 {
            K_V[i] -= 16.0;
        }

        K_V[i] /= (2.0 as f32).powi(K_V_scale as i32);
    }

    return K_V;
}

fn calc_K_Ta() -> [f32; PIXEL_COUNT] {
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

    let mut K_Ta: [f32; PIXEL_COUNT] = [0.0; PIXEL_COUNT];

    for i in 0..PIXEL_COUNT {
        K_Ta[i] = K_Ta_RC_EE[i] as f32;
        K_Ta[i] += K_Ta_EE[i] as f32 * (2.0 as f32).powi(K_Ta_scale2 as i32);
        K_Ta[i] /= (2.0 as f32).powi(K_Ta_scale1 as i32);
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
    *Ks_To1 = Ks_To1_EE >> Ks_To_scale;

    let mut Ks_To2_EE: i16 = ((get_eeprom_val(0x243D) & 0xFF00) >> 8) as i16;
    if Ks_To2_EE > 127 { Ks_To2_EE -= 256 }
    *Ks_To2 = Ks_To2_EE >> Ks_To_scale;

    let mut Ks_To3_EE: i16 = (get_eeprom_val(0x243E) & 0x00FF) as i16;
    if Ks_To3_EE > 127 { Ks_To3_EE -= 256 }
    *Ks_To3 = Ks_To3_EE >> Ks_To_scale;

    let mut Ks_To4_EE: i16 = ((get_eeprom_val(0x243E) & 0xFF00) >> 8) as i16;
    if Ks_To4_EE > 127 { Ks_To4_EE -= 256 }
    *Ks_To4 = Ks_To4_EE >> Ks_To_scale;
}

fn calc_Alpha_corr_range1(Ks_To1: i16) -> f32 {
    return 1.0 / (1.0 + Ks_To1 * 40.0);
}

fn calc_Alpha_corr_range2() -> f32 {
    return 1.0;
}

fn calc_Alpha_corr_range3(Ks_To2: i16, CT3: i16) -> f32 {
    return 1.0 + Ks_To2 * CT3;
}

fn calc_Alpha_corr_range4(Ks_To2: i16, Ks_To3: i16, CT3: i16, CT4: i16) -> f32 {
    return (1.0 + Ks_To2 * CT3) * (1.0 + Ks_To3 * (CT4 - CT3));
}

fn calc_a_CP(a_CP_0: &mut i16, a_CP_1: &mut i16) {
    let a_scale_CP = ((get_eeprom_val(0x2420) & 0xF000) >> 12) as i16 + 27;
    let mut CP_P1_P0_ratio = ((get_eeprom_val(0x2439) & 0xFC00) >> 10) as i16;
    if CP_P1_P0_ratio > 31 {
        CP_P1_P0_ratio -= 64;
    }

    *a_CP_0 = ((get_eeprom_val(0x2439) & 0x03FF)) as i16 >> a_scale_CP;
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

    let K_V_CP: i16 = K_V_CP_EE >> K_V_Scale;
    return K_V_CP;
}

fn calc_K_Ta_CP() -> i16 {
    let K_Ta_scale_1: u16 = ((get_eeprom_val(0x2438) & 0x00F0) as u16 >> 4) + 8;

    let mut K_Ta_CP_EE: i16 = (get_eeprom_val(0x243B) & 0x00FF) as i16;
    if K_Ta_CP_EE > 127 {
        K_Ta_CP_EE -= 256;
    }

    let K_Ta_CP = K_Ta_CP_EE >> K_Ta_scale_1;
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
