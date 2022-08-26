use phf::phf_map;

pub mod p1;
pub mod p2;
pub mod p3;
pub mod p4;
pub mod p5;
pub mod p6;
pub mod p7;
pub mod p8;
pub mod p9;
pub mod p10;
pub mod p11;
pub mod p12;
pub mod p13;
pub mod p14;
pub mod p15;
pub mod p16;
pub mod p17;
pub mod p18;
pub mod p19;
pub mod p20;
pub mod p21;
pub mod p22;
pub mod p35;

pub const SOLUTION_FUNCTIONS_HASHMAP: phf::Map<u32, fn() -> u64> = phf_map! {
     1u32 =>  p1::s_v1,
     2u32 =>  p2::s_v1,
     3u32 =>  p3::s_v1,
     4u32 =>  p4::s_v1,
     5u32 =>  p5::s_v1,
     6u32 =>  p6::s_v1,
     7u32 =>  p7::s_v1,
     8u32 =>  p8::s_v1,
     9u32 =>  p9::s_v1,
    10u32 => p10::s_v1,
    11u32 => p11::s_v1,
    12u32 => p12::s_v2,
    13u32 => p13::s_v1,
    14u32 => p14::s_v1,
    15u32 => p15::s_v1,
    16u32 => p16::s_v1,
    17u32 => p17::s_v1,
    18u32 => p18::s_v1,
    19u32 => p19::s_v1,
    20u32 => p20::s_v1,
    21u32 => p21::s_v1,
    22u32 => p22::s_v1,
    35u32 => p35::s_v1,
};
