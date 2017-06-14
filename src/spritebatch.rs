pub enum SpriteSortMode
{
    /// <summary>
    /// All sprites are drawing when <see cref="SpriteBatch.End"/> invokes, in order of draw call sequence. Depth is ignored.
    /// </summary>
    SpriteSortModeDeferred,
    /// <summary>
    /// Each sprite is drawing at individual draw call, instead of <see cref="SpriteBatch.End"/>. Depth is ignored.
    /// </summary>
    SpriteSortModeImmediate,
    /// <summary>
    /// Same as <see cref="SpriteSortMode.Deferred"/>, except sprites are sorted by texture prior to drawing. Depth is ignored.
    /// </summary>
    SpriteSortModeTexture,
    /// <summary>
    /// Same as <see cref="SpriteSortMode.Deferred"/>, except sprites are sorted by depth in back-to-front order prior to drawing.
    /// </summary>
    SpriteSortModeBackToFront,
    /// <summary>
    /// Same as <see cref="SpriteSortMode.Deferred"/>, except sprites are sorted by depth in front-to-back order prior to drawing.
    /// </summary>
    SpriteSortModeFrontToBack
}

pub struct SpriteBatch {

}