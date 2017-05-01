// Store 8 bit value in reg
macro_rules! store_reg (($c:expr; $reg:ident; $value:expr) => {
    $c.$reg = $value;
});

// Store 16 bit value in regs
macro_rules! store_reg16 (($c:expr; $reg1:ident, $reg2:ident; $value:expr) => {
    let lower = ($value & 0xFF) as u8;
    let upper = ($value >> 8) as u8;
    $c.$reg1 = upper;
    $c.$reg2 = lower;
});

macro_rules! inc_reg (($c: expr; $reg: ident) => {
    use cpu::macros::half_carry_add;
    if half_carry_add($c.$reg, 1) {
        $c.H = 1;
    }
    $c.$reg = $c.$reg.wrapping_add(1);
    if $c.$reg == 0 {
        $c.Z = 0;
    }
    $c.N = 0;
    $c.step();
    $c.pc += 1;
});

macro_rules! dec_reg (($c: expr; $reg: ident) => {
    use cpu::macros::half_carry_sub;

    if half_carry_sub($c.$reg, 1) {
        $c.H = 1;
    }

    $c.$reg = $c.$reg.wrapping_sub(1);

    if $c.$reg == 0 {
        $c.Z = 1;
    }
    $c.N = 1; // 
    $c.pc += 1;
    $c.step();

});

macro_rules! get_reg16 (($c:expr; $reg1:ident, $reg2:ident) => {
    {
        let value = (($c.$reg1 as u16) << 8) | $c.$reg2 as u16;
        value
    }
});

macro_rules! inc_reg16 (($c:expr; $reg1:ident, $reg2:ident) => {
    {
        let mut value = get_reg16!($c; $reg1, $reg2);
        value = value.wrapping_add(1);
        value
    }
});

macro_rules! load_from_reg (($c:expr; $dst:ident, $src:ident) => {
    $c.$dst = $c.$src;
});

pub fn half_carry_add(initial: u8, value: u8) -> bool {
    let a = initial & 0xF;
    let b = value & 0xF;
    return (a + b) & 0x10 == 0x10;
}

pub fn half_carry_sub(initial: u8, value: u8) -> bool {
    let a = initial & 0xF;
    let b = value & 0xF;
    return (a.wrapping_sub(b)) & 0x10 == 0x10;
}
