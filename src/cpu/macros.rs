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
    $c.$reg = $c.$reg.wrapping_add(1)
});

macro_rules! dec_reg (($c: expr; $reg: ident) => {
    $c.$reg = $c.$reg.wrapping_sub(1)
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
