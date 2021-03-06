//! # Art
//!
//! 一个用来建模艺术概念的代码库
//!
// 通过下面的pub use，调整了对用户暴露的路径，用户可以通过art::PrimaryColor和art::mix调用方法
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    /// RYB颜色模型的三原色
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// RYB模型的调和色
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use super::kinds::*;

    /// 将两种等量的原色混合生成调和色
    pub fn mix(_c1: PrimaryColor, _c2: PrimaryColor) -> SecondaryColor {
        // --略--
        SecondaryColor::Green
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let red = PrimaryColor::Red;
        let yellow = PrimaryColor::Yellow;
        mix(red, yellow);
    }
}