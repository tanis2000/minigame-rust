////////////////////////////////////////////////////////
/// \brief Enumeration of the blending factors
///
/// The factors are mapped directly to their OpenGL equivalents,
/// specified by glBlendFunc() or glBlendFuncSeparate().
////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub enum Factor {
    Zero, //< (0, 0, 0, 0)
    One, //< (1, 1, 1, 1)
    SrcColor, //< (src.r, src.g, src.b, src.a)
    OneMinusSrcColor, //< (1, 1, 1, 1) - (src.r, src.g, src.b, src.a)
    DstColor, //< (dst.r, dst.g, dst.b, dst.a)
    OneMinusDstColor, //< (1, 1, 1, 1) - (dst.r, dst.g, dst.b, dst.a)
    SrcAlpha, //< (src.a, src.a, src.a, src.a)
    OneMinusSrcAlpha, //< (1, 1, 1, 1) - (src.a, src.a, src.a, src.a)
    DstAlpha, //< (dst.a, dst.a, dst.a, dst.a)
    OneMinusDstAlpha, //< (1, 1, 1, 1) - (dst.a, dst.a, dst.a, dst.a)
}

////////////////////////////////////////////////////////
/// \brief Enumeration of the blending equations
///
/// The equations are mapped directly to their OpenGL equivalents,
/// specified by glBlendEquation() or glBlendEquationSeparate().
////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub enum Equation {
    Add, //< Pixel = Src * SrcFactor + Dst * DstFactor
    Subtract, //< Pixel = Src * SrcFactor - Dst * DstFactor
}

pub struct BlendMode {
    pub color_src_factor: Factor, //< Source blending factor for the color channels
    pub color_dst_factor: Factor, //< Destination blending factor for the color channels
    pub color_equation: Equation, //< Blending equation for the color channels
    pub alpha_src_factor: Factor, //< Source blending factor for the alpha channel
    pub alpha_dst_factor: Factor, //< Destination blending factor for the alpha channel
    pub alpha_equation: Equation, //< Blending equation for the alpha channel
}

impl BlendMode {
    fn new() -> BlendMode {
        BlendMode {
            color_src_factor: Factor::SrcAlpha,
            color_dst_factor: Factor::OneMinusSrcAlpha,
            color_equation: Equation::Add,
            alpha_src_factor: Factor::One,
            alpha_dst_factor: Factor::OneMinusSrcAlpha,
            alpha_equation: Equation::Add,
        }
    }

    fn color_src_factor(&mut self, value: Factor) -> &mut BlendMode {
        self.color_src_factor = value;
        self
    }

    fn color_dst_factor(&mut self, value: Factor) -> &mut BlendMode {
        self.color_dst_factor = value;
        self
    }

    fn color_equation(&mut self, value: Equation) -> &mut BlendMode {
        self.color_equation = value;
        self
    }

    fn alpha_src_factor(&mut self, value: Factor) -> &mut BlendMode {
        self.alpha_src_factor = value;
        self
    }

    fn alpha_dst_factor(&mut self, value: Factor) -> &mut BlendMode {
        self.alpha_dst_factor = value;
        self
    }

    fn alpha_equation(&mut self, value: Equation) -> &mut BlendMode {
        self.alpha_equation = value;
        self
    }

    fn build(&self) -> BlendMode {
        BlendMode {
            color_src_factor: self.color_src_factor,
            color_dst_factor: self.color_dst_factor,
            color_equation: self.color_equation,
            alpha_src_factor: self.alpha_src_factor,
            alpha_dst_factor: self.alpha_dst_factor,
            alpha_equation: self.alpha_equation,
        }
    }
}

pub const BLEND_ALPHA: BlendMode = BlendMode {
    color_src_factor: Factor::SrcAlpha,
    color_dst_factor: Factor::OneMinusSrcAlpha,
    color_equation: Equation::Add,
    alpha_src_factor: Factor::One,
    alpha_dst_factor: Factor::OneMinusSrcAlpha,
    alpha_equation: Equation::Add,
};
