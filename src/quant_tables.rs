use crate::quant_tables::TableType::{Chrominance, Luminance};
use std::simd::Simd;

#[derive(Debug)]
enum Precision {
    EightBit,
    SixteenBit,
}

#[derive(Debug)]
enum TableType {
    Luminance = 0,
    Chrominance = 1,
}

// 8x8
pub const QUANT_TABLE_WIDTH: usize = 8;

#[derive(Debug)]
pub struct QuantTable {
    table_type: TableType,
    precision: Precision,
    data: Simd<u8, 64>, // 8x8
}

impl QuantTable {
    pub(crate) fn from(qt_id: u8, qt_precision: u8, qt_data: Simd<u8, 64>) -> Self {
        QuantTable {
            table_type: match qt_id {
                0 => Luminance,
                1 => Chrominance,
                _ => unreachable!(),
            },
            precision: match qt_precision {
                0 => Precision::EightBit,
                1 => Precision::SixteenBit,
                _ => unreachable!(),
            },
            data: qt_data,
        }
    }


}