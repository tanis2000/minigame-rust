use graphicsdevice::GraphicsDevice;
use spritebatchitem::SpriteBatchItem;
use spritebatch::SpriteSortMode;
use renderstate::RenderState;
use texture::Texture;
use log::Log;
use std::i32;
use std::rc::Rc;
use vertexpositioncolortexture::VertexPositionColorTexture;

pub struct SpriteBatcher {
    initial_batch_size: i32,
    max_batch_size: i32,
    initial_vertex_array_size: i32,
    //graphics_device: &'a GraphicsDevice,
    batch_item_list: Vec<SpriteBatchItem>, /// The list of batch items to process.
    batch_item_count: i32, /// Index pointer to the next available SpriteBatchItem in _batchItemList.
    index: Vec<i32>, /// Vertex index array. The values in this array never change.
    vertex_array: Vec<VertexPositionColorTexture>,
}

impl SpriteBatcher {
    pub fn new(/*, graphics_device: &'a GraphicsDevice*/) -> Self {
        let mut bil = Vec::new();
        for _i in 0..256 {
            bil.push(SpriteBatchItem::new());
        }
        
        let mut sb = SpriteBatcher {
            initial_batch_size: 256,
            max_batch_size: i32::MAX / 6, // 6 = 4 vertices unique and 2 shared, per quad
            initial_vertex_array_size: 256, 
            //graphics_device: graphics_device,
            batch_item_list: bil,
            batch_item_count: 0,
            index: Vec::new(),
            vertex_array: Vec::new(),
        };

        sb.ensure_array_capacity(256);

        sb
    }

    pub fn create_batch_item(&mut self) -> &mut SpriteBatchItem {
        if self.batch_item_count >= self.batch_item_list.len() as i32 {
            let old_size = self.batch_item_list.len();
            let mut new_size = old_size + old_size / 2; // grow by x1.5
            new_size = (new_size + 63) & (!63);        // grow in chunks of 64.
            self.batch_item_list.resize(new_size as usize, SpriteBatchItem::new());
        }
        let item = &mut self.batch_item_list[self.batch_item_count as usize];
        self.batch_item_count = self.batch_item_count + 1;
        item
    }

    pub fn ensure_array_capacity(&mut self, num_batch_items: i32) {
        let needed_capacity = 6 * num_batch_items;
        if needed_capacity <= self.index.len() as i32 {
            // Short circuit out of here because we have enough capacity.
            return;
        }

        let mut new_index: Vec<i32> = Vec::with_capacity(needed_capacity as usize);

        for i in 0..self.index.len() as usize {
            new_index.push(self.index[i]);
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
            new_index.insert((i * 6 + 0) as usize, (i * 4) as i32);
            new_index.insert((i * 6 + 1) as usize, (i * 4 + 1) as i32);
            new_index.insert((i * 6 + 2) as usize, (i * 4 + 2) as i32);
            // Triangle 2
            new_index.insert((i * 6 + 3) as usize, (i * 4 + 1) as i32);
            new_index.insert((i * 6 + 4) as usize, (i * 4 + 3) as i32);
            new_index.insert((i * 6 + 5) as usize, (i * 4 + 2) as i32);
        }
        self.index = new_index;

        self.vertex_array.resize(needed_capacity as usize, VertexPositionColorTexture::new());
    }

    pub fn draw_batch(&mut self, sort_mode: SpriteSortMode/*, Effect effect*/, render_state: &mut RenderState, graphics_device: &mut GraphicsDevice) {
        Log::debug("draw_batch: batch_item_count follows");
        Log::debug(&self.batch_item_count.to_string());

        // nothing to do
        if self.batch_item_count == 0 {
            return;
        }

        // sort the batch items
        match sort_mode {
            SpriteSortMode::SpriteSortModeTexture => self.batch_item_list.sort_by(|a, b| a.cmp(b)),
            SpriteSortMode::SpriteSortModeFrontToBack => self.batch_item_list.sort_by(|a, b| a.cmp(b)),
            SpriteSortMode::SpriteSortModeBackToFront => self.batch_item_list.sort_by(|a, b| a.cmp(b)),
            _ => self.batch_item_list.sort_by(|a, b| a.cmp(b)),
        }

        // Determine how many iterations through the drawing code we need to make
        let mut batch_index: i32 = 0;
        let mut batch_count: i32 = self.batch_item_count;

        // Iterate through the batches, doing short.MaxValue sets of vertices only.
        while batch_count > 0 {
            // setup the vertexArray array
            let mut start_index: i32 = 0;
            let mut index: i32 = 0;
            let mut tex: Option<Rc<Texture>> = None;

            let mut num_batches_to_process: i32 = batch_count;
            if num_batches_to_process > self.max_batch_size {
                num_batches_to_process = self.max_batch_size;
            }
            
            {
                self.ensure_array_capacity(num_batches_to_process);
            }

            // Draw the batches
            for _i in 0..num_batches_to_process {
                // if the texture changed, we need to flush and bind the new texture
                let mut should_flush: bool;
                Log::debug("batch index follows");
                Log::debug(&batch_index.to_string());
                if self.batch_item_list[batch_index as usize].texture.is_some() {
                    Log::debug("has batch item list texture");
                }
                if tex.is_some() {
                    Log::debug("has tex");
                }

                if self.batch_item_list[batch_index as usize].texture.is_some() && tex.is_none() {
                    should_flush = true;
                } else if self.batch_item_list[batch_index as usize].texture.is_none() && tex.is_some() {
                    should_flush = true;
                } else if self.batch_item_list[batch_index as usize].texture.is_none() && tex.is_none() {
                    should_flush = false;
                } else {
                    should_flush = &**self.batch_item_list[batch_index as usize].texture.as_ref().unwrap() as *const _ != &**tex.as_ref().unwrap() as *const _;
                    //let a = self.batch_item_list[batch_index as usize].texture.unwrap();
                }
                //let b:() = &**tex.as_ref().unwrap();
                if should_flush {
                    self.flush_vertex_array(start_index, index /*, effect*/, tex, render_state, graphics_device);

                    tex = self.batch_item_list[batch_index as usize].texture.clone();
                    start_index = 0;
                    index = 0;
                }

                let mut item = &mut self.batch_item_list[batch_index as usize];
                // store the SpriteBatchItem data in our vertexArray
                self.vertex_array[index as usize] = item.vertex_tl;
                index = index + 1;
                self.vertex_array[index as usize] = item.vertex_tr;
                index = index + 1;
                self.vertex_array[index as usize] = item.vertex_bl;
                index = index + 1;
                self.vertex_array[index as usize] = item.vertex_tr;
                index = index + 1;
                self.vertex_array[index as usize] = item.vertex_br;
                index = index + 1;
                self.vertex_array[index as usize] = item.vertex_bl;
                index = index + 1;

                Log::debug("SpriteBatcher::draw_batch()");
                //Log::debug("{:?}", self.vertex_array[(index-6) as usize].position);
                //Log::debug("{:?}", self.vertex_array[(index-5) as usize].position);
                //Log::debug("{:?}", self.vertex_array[(index-4) as usize].position);
                //Log::debug("{:?}", self.vertex_array[(index-3) as usize].position);
                //Log::debug("{:?}", self.vertex_array[(index-2) as usize].position);
                //Log::debug("{:?}", self.vertex_array[(index-1) as usize].position);

                // Release the texture.
                item.set_texture(None);
                batch_index += 1;
            }
            // flush the remaining vertexArray data
            self.flush_vertex_array(start_index, index /*, effect*/, tex, render_state, graphics_device);
            // Update our batch count to continue the process of culling down
            // large batches
            batch_count -= num_batches_to_process;
        }
        // return items to the pool.
        self.batch_item_count = 0;
    }

    pub fn flush_vertex_array(&mut self, start: i32, end: i32 /*, Effect effect*/, texture: Option<Rc<Texture>>, render_state: &mut RenderState, graphics_device: &mut GraphicsDevice) {
        if start == end {
            return;
        }

        let vertex_count: i32 = end - start;
        render_state.set_texture(texture);

        //Log::debug("SpriteBatcher::flush_vertex_array");
        //Log::debug("{:?}", self.vertex_array);
        graphics_device.draw(&self.vertex_array, vertex_count, render_state);
    }
  

}