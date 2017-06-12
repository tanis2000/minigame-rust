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
    pub colorSrcFactor: Factor, //< Source blending factor for the color channels
    pub colorDstFactor: Factor, //< Destination blending factor for the color channels
    pub colorEquation: Equation, //< Blending equation for the color channels
    pub alphaSrcFactor: Factor, //< Source blending factor for the alpha channel
    pub alphaDstFactor: Factor, //< Destination blending factor for the alpha channel
    pub alphaEquation: Equation, //< Blending equation for the alpha channel
}

impl BlendMode {
    fn new() -> BlendMode {
        BlendMode {
            colorSrcFactor: Factor::SrcAlpha,
            colorDstFactor: Factor::OneMinusSrcAlpha,
            colorEquation: Equation::Add,
            alphaSrcFactor: Factor::One,
            alphaDstFactor: Factor::OneMinusSrcAlpha,
            alphaEquation: Equation::Add,
        }
    }

    fn colorSrcFactor(&mut self, value: Factor) -> &mut BlendMode {
        self.colorSrcFactor = value;
        self
    }

    fn colorDstFactor(&mut self, value: Factor) -> &mut BlendMode {
        self.colorDstFactor = value;
        self
    }

    fn colorEquation(&mut self, value: Equation) -> &mut BlendMode {
        self.colorEquation = value;
        self
    }

    fn alphaSrcFactor(&mut self, value: Factor) -> &mut BlendMode {
        self.alphaSrcFactor = value;
        self
    }

    fn alphaDstFactor(&mut self, value: Factor) -> &mut BlendMode {
        self.alphaDstFactor = value;
        self
    }

    fn alphaEquation(&mut self, value: Equation) -> &mut BlendMode {
        self.alphaEquation = value;
        self
    }

    fn build(&self) -> BlendMode {
        BlendMode {
            colorSrcFactor: self.colorSrcFactor,
            colorDstFactor: self.colorDstFactor,
            colorEquation: self.colorEquation,
            alphaSrcFactor: self.alphaSrcFactor,
            alphaDstFactor: self.alphaDstFactor,
            alphaEquation: self.alphaEquation,
        }
    }
}

pub const BlendAlpha: BlendMode = BlendMode {
    colorSrcFactor: Factor::SrcAlpha,
    colorDstFactor: Factor::OneMinusSrcAlpha,
    colorEquation: Equation::Add,
    alphaSrcFactor: Factor::One,
    alphaDstFactor: Factor::OneMinusSrcAlpha,
    alphaEquation: Equation::Add,
};
