use tobj::LoadError;

use crate::base::vertex::{VertexLayout};

pub struct Mesh<T: VertexLayout>{
    pub vertices: Vec<T>,
    pub indices: Vec<u16>,
}

impl<T: VertexLayout> Mesh<T>{
    pub fn from_obj(filename: &str) -> Result<Self,LoadError>{
        let mut vertices = vec![];
        let mut indices: Vec<u16> = vec![];

        let (models,materials) = tobj::load_obj(filename,&tobj::GPU_LOAD_OPTIONS)?;

        for (i,model) in models.iter().enumerate(){
            let mesh = &model.mesh;

            for j in 0..mesh.indices.len(){

                let mut vertex = T::new();

                let idx = mesh.indices[j] as usize;


                let position = [
                    mesh.positions[3*idx],
                    mesh.positions[3*idx+1],
                    mesh.positions[3*idx+2]
                ];

                vertex.add_position(position);


                let mut normal: [f32;3]  = [0.0,0.0,0.0];
                let mut uv: [f32;2] = [0.0,0.0];

                if !mesh.normals.is_empty(){
                    normal[0] = mesh.normals[3*idx];
                    normal[1] = mesh.normals[3*idx + 1];
                    normal[2] = mesh.normals[3*idx + 2];
                    vertex.try_add_normal(normal).or(Err(LoadError::NormalParseError))?;
                }

                if !mesh.texcoords.is_empty(){
                    uv[0] = mesh.texcoords[2*idx];
                    uv[1] = mesh.texcoords[2*idx + 1];
                    vertex.try_add_uv(uv).or(Err(LoadError::TexcoordParseError))?;
                }

                vertices.push(vertex);
                indices.push(j as u16);

            }
        }

        Ok(Mesh{
            vertices,
            indices,
        })
    }

    pub fn num_indices(&self) -> u32{
        self.indices.len() as u32
    }
}