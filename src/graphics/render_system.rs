use super::super::components;
use super::renderer;

#[derive(Debug)]
pub struct RenderCommand {
    pub position: na::Vector3<f32>,
    pub rotation: f32,
    pub scale: na::Vector3<f32>,
    pub tex_id: gl::types::GLuint,
    pub coords : na::Vector4<f32>
}

impl RenderCommand {
    pub fn new(
        _pos: na::Vector3<f32>,
        _rot: f32,
        _scale: na::Vector3<f32>,
        _tex_id: gl::types::GLuint,
        _coords : na::Vector4<f32>
    ) -> RenderCommand {
        RenderCommand {
            position: _pos,
            rotation: _rot,
            scale: _scale,
            tex_id: _tex_id,
            coords : _coords
        }
    }
}
pub struct RenderSystem<'a> {
    pub renderer: &'a mut renderer::Renderer,
    pub render_commands: Vec<RenderCommand>,
    pub vertices_list: [renderer::Vertex; 4],
    pub modelview_matrix : na::Matrix4<f32>
}
impl<'a> RenderSystem<'a> {
    pub fn init(&mut self) {
        self.renderer.init();
    }
}
impl <'a> RenderSystem<'_>{
    pub fn modify_texcoords(slice : &mut [renderer::Vertex],sheet_size : na::Vector2<f32>,coords : na::Vector4<f32>) {

        let xTex = 1.0f32 - (coords.x / sheet_size.x);
        let wTex = coords.z / sheet_size.x;
        let yTex = 1.0f32 - (coords.y / sheet_size.y);
        let hTex = coords.w / sheet_size.y;

        (*slice)[0].tex_coord = na::Vector3::<f32>::new(xTex - wTex, yTex,0.0);
        (*slice)[1].tex_coord = na::Vector3::<f32>::new(xTex,yTex ,0.0);
        (*slice)[2].tex_coord = na::Vector3::<f32>::new(xTex,yTex - hTex ,0.0);
        (*slice)[3].tex_coord = na::Vector3::<f32>::new(xTex - wTex,yTex - hTex ,0.0);
        //*slice = &mut value[0..];
        //value[0].tex_coord =

    }

}
impl<'a> RenderSystem<'a> {
    //X : posx , Y : posy , Z : scalex , W:scaleY ,
    pub fn new(renderer: &mut renderer::Renderer) -> RenderSystem {
        let ortho: na::Matrix4<f32> = *nalgebra::Orthographic3::new(
            -1.0,
            (1000.0 - 1.0) + 1.0,
            -1.0,
            (1000.0 - 1.0) + 1.0,
            -1000.0,
            1000.0,
        ).as_matrix();

        RenderSystem {
            renderer: renderer,
            render_commands: vec![],
            vertices_list: [
                renderer::Vertex::new(
                    na::Vector3::<f32>::new(1.0, 1.0, 1.0),
                    na::Vector3::<f32>::new(1.0, 1.0, 1.0),
                ),
                renderer::Vertex::new(
                    na::Vector3::<f32>::new(-1.0, 1.0, 1.0),
                    na::Vector3::<f32>::new(0.0, 1.0, 1.0),
                ),
                renderer::Vertex::new(
                    na::Vector3::<f32>::new(-1.0, -1.0, 1.0),
                    na::Vector3::<f32>::new(0.0, 0.0, 1.0),
                ),
                renderer::Vertex::new(
                    na::Vector3::<f32>::new(1.0, -1.0, 1.0),
                    na::Vector3::<f32>::new(1.0, 0.0, 1.0),
                ),
            ],
            modelview_matrix : ortho
        }
    }
    pub fn render(&mut self) {
        for rendercommand in &mut self.render_commands {
            let mut local_vertices = self.vertices_list.clone();
            let mut model = na::Matrix4::new_translation(&na::Vector3::<f32>::new(rendercommand.position.x.round(),rendercommand.position.y.round(),rendercommand.position.z.round()));
            model =
                model * na::Matrix4::from_scaled_axis(&na::Vector3::z() * rendercommand.rotation);

            model = model.prepend_nonuniform_scaling(&(rendercommand.scale));

            let modelviewprojection = self.modelview_matrix * (model);
            for x in 0..4 {
                let vertexpos = self.vertices_list[x].position;
                let transformed_vertex = modelviewprojection
                    * na::Point4::new(vertexpos.x, vertexpos.y, vertexpos.z, 1.0);

                local_vertices[x].position = na::Vector3::<f32>::new(
                    transformed_vertex.x,
                    transformed_vertex.y,
                    transformed_vertex.z,
                );
            }
            if(rendercommand.coords != na::Vector4::<f32>::zeros()){
               RenderSystem::modify_texcoords(&mut local_vertices,na::Vector2::<f32>::new(512.0,512.0),rendercommand.coords);
            }

            self.renderer
                .submit(&mut local_vertices, rendercommand.tex_id);
        }
    }
    pub fn flush(&mut self) {
        self.renderer.end();

        self.render_commands.clear();
    }
}

pub struct RenderCommandsSystem<'a, 'b> {
    pub m_renderer: &'a mut RenderSystem<'b>,
}
use specs::{Join, ReadStorage, System};

impl<'a, 'b, 'c> System<'a> for RenderCommandsSystem<'b, 'c> {
    type SystemData = (
        ReadStorage<'a, components::Transform>,
        ReadStorage<'a, components::Sprite>,
    );
    fn run(&mut self, (transform, sprite): Self::SystemData) {
        for (pos, sprite) in (&transform, &sprite).join() {
            self.m_renderer.render_commands.push(RenderCommand::new(
                (*pos).position,
                (*pos).rotation,
                (*pos).scale,
                (*sprite).tex_id,
                (*sprite).coords
            ));
        }
    }
}
