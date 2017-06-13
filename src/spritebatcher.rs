use sdl2::render::RendererContext;
use sdl2::render::Canvas;
use graphicsdevice::GraphicsDevice;
use spritebatchitem::SpriteBatchItem;
use std::i32;
use std::vec;
use vertexpositioncolortexture::VertexPositionColorTexture;

pub struct SpriteBatcher<'a> {
    initial_batch_size: i32,
    max_batch_size: i32,
    initial_vertex_array_size: i32,
    //renderer: &'a RendererContext<Canvas>,
    graphics_device: &'a GraphicsDevice,
    batch_item_list: Vec<SpriteBatchItem<'a>>, /// The list of batch items to process.
    batch_item_count: i32, /// Index pointer to the next available SpriteBatchItem in _batchItemList.
    index: Vec<i32>, /// Vertex index array. The values in this array never change.
    vertex_array: Vec<VertexPositionColorTexture>,
}

impl<'a> SpriteBatcher<'a> {
    pub fn new(/*renderer: &'a RendererContext,*/ graphics_device: &'a GraphicsDevice) -> SpriteBatcher<'a> {
        let mut bil = Vec::new();
        for i in 0..256 {
            bil.push(SpriteBatchItem::new());
        }
        
        let mut sb = SpriteBatcher {
            initial_batch_size: 256,
            max_batch_size: i32::MAX / 6, // 6 = 4 vertices unique and 2 shared, per quad
            initial_vertex_array_size: 256, 
            //renderer: renderer,
            graphics_device: graphics_device,
            batch_item_list: bil,
            batch_item_count: 0,
            index: Vec::new(),
            vertex_array: Vec::new(),
        };

        sb.ensure_array_capacity(256);

        sb
    }

    pub fn create_batch_item() -> SpriteBatchItem<'a> {
        SpriteBatchItem::new()
    }

    pub fn ensure_array_capacity(&mut self, num_batch_items: i32) {
        let neededCapacity = 6 * num_batch_items;
        if neededCapacity <= self.index.len() as i32 {
            // Short circuit out of here because we have enough capacity.
            return;
        }

        let mut newIndex: Vec<i32> = Vec::with_capacity(neededCapacity as usize);
        let start = 0;

        for i in 0..self.index.len() as usize {
            newIndex.push(self.index[i]);
        }

        let start = self.index.len() / 6;

        for i in start..num_batch_items as usize {
            /*
            *  TL    TR
            *   0----1 0,1,2,3 = index offsets for vertex indices
            *   |   /| TL,TR,BL,BR are vertex references in SpriteBatchItem.
            *   |  / |
            *   | /  |
            *   |/   |
            *   2----3
            *  BL    BR
            */
            // Triangle 1
            newIndex.insert((i * 6 + 0) as usize, (i * 4) as i32);
            newIndex.insert((i * 6 + 1) as usize, (i * 4 + 1) as i32);
            newIndex.insert((i * 6 + 2) as usize, (i * 4 + 2) as i32);
            // Triangle 2
            newIndex.insert((i * 6 + 3) as usize, (i * 4 + 1) as i32);
            newIndex.insert((i * 6 + 4) as usize, (i * 4 + 3) as i32);
            newIndex.insert((i * 6 + 5) as usize, (i * 4 + 2) as i32);
        }
        self.index = newIndex;

        self.vertex_array.resize(neededCapacity as usize, VertexPositionColorTexture::new());
        
    }
}