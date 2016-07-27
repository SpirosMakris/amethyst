//! This module provides an asset managar
//! which loads and provides access to assets,
//! such as `Texture`s, `Mesh`es and `Fragment`s.

extern crate amethyst_renderer;
extern crate gfx_device_gl;
extern crate gfx;

use self::gfx::traits::FactoryExt;
use self::gfx::Factory;
use self::gfx::format::{Formatted, SurfaceTyped};
use self::amethyst_renderer::VertexPosNormal;
use self::amethyst_renderer::target::ColorFormat;
use std::collections::HashMap;
use renderer::{Fragment, Texture, Kind, TextureImpl, FragmentImpl};

pub enum FactoryImpl {
    OpenGL {
        factory: gfx_device_gl::Factory,
    },
    #[cfg(windows)]
    Direct3D {
        // stub
    },
    Null,
}

#[derive(Clone)]
pub enum MeshImpl {
    OpenGL {
        buffer: gfx::handle::Buffer<gfx_device_gl::Resources, VertexPosNormal>,
        slice: gfx::Slice<gfx_device_gl::Resources>,
    },
    #[cfg(windows)]
    Direct3D {
        // stub
    },
    Null,
}

#[derive(Clone)]
pub struct Mesh {
    mesh_impl: MeshImpl,
}

pub struct AssetManager {
    factory_impl: FactoryImpl,
    meshes: HashMap<String, Mesh>,
    textures: HashMap<String, Texture>,
}

impl AssetManager {
    /// Create a new `AssetManager` from `FactoryImpl` (used internally).
    pub fn new(factory_impl: FactoryImpl) -> AssetManager {
        AssetManager {
            factory_impl: factory_impl,
            meshes: HashMap::new(),
            textures: HashMap::new(),
        }
    }
    /// Load a `Mesh` from vertex data.
    pub fn load_mesh(&mut self, name: &str, data: &Vec<VertexPosNormal>) {
        match self.factory_impl {
            FactoryImpl::OpenGL {
                ref mut factory
            } => {
                let (buffer, slice) = factory.create_vertex_buffer_with_slice(&data, ());
                let mesh_impl =
                    MeshImpl::OpenGL {
                        buffer: buffer,
                        slice: slice,
                    };
                let mesh = Mesh {
                    mesh_impl: mesh_impl,
                };
                self.meshes.insert(name.into(), mesh);
            },
            #[cfg(windows)]
            FactoryImpl::Direct3D {  } => {
                unimplemented!();
            },
            FactoryImpl::Null => (),
        }
    }
    /// Lookup a `Mesh` by name.
    pub fn get_mesh(&mut self, name: &str) -> Option<Mesh> {
        match self.meshes.get(name.into()) {
            Some(mesh) => {
                Some((*mesh).clone())
            },
            None => None,
        }
    }
    /// Load a `Texture` from pixel data.
    pub fn load_texture(&mut self, name: &str, kind: Kind, data: &[&[<<ColorFormat as Formatted>::Surface as SurfaceTyped>::DataType]]) {
        match self.factory_impl {
            FactoryImpl::OpenGL {
                ref mut factory
            } => {
                let shader_resource_view = match factory.create_texture_const::<ColorFormat>(kind, data) {
                    Ok((_, shader_resource_view)) => shader_resource_view,
                    Err(_) => return,
                };
                let texture = amethyst_renderer::Texture::Texture(shader_resource_view);
                let texture_impl = TextureImpl::OpenGL {
                    texture: texture,
                };
                let texture = Texture {
                    texture_impl: texture_impl,
                };
                self.textures.insert(name.into(), texture);
            },
            #[cfg(windows)]
            FactoryImpl::Direct3D {  } => {
                unimplemented!();
            },
            FactoryImpl::Null => (),
        }
    }
    /// Create a constant solid color `Texture` from a specified color.
    pub fn create_constant_texture(&mut self, name: &str, color: [f32; 4]) {
        let texture = amethyst_renderer::Texture::Constant(color);
        let texture_impl = TextureImpl::OpenGL {
            texture: texture,
        };
        let texture = Texture {
            texture_impl: texture_impl,
        };
        self.textures.insert(name.into(), texture);
    }
    /// Lookup a `Texture` by name.
    pub fn get_texture(&mut self, name: &str) -> Option<Texture> {
        match self.textures.get(name.into()) {
            Some(texture) => {
                Some((*texture).clone())
            },
            None => None,
        }
    }
    /// Construct and return a `Fragment` from previously loaded mesh, ka and kd textures and a transform matrix.
    pub fn get_fragment(&mut self, mesh: &str, ka: &str, kd: &str, transform: [[f32; 4]; 4]) -> Option<Fragment> {
        let mesh = self.get_mesh(mesh).unwrap();
        let ka = self.get_texture(ka).unwrap();
        let kd = self.get_texture(kd).unwrap();
        match self.factory_impl {
            FactoryImpl::OpenGL {
                ..
            } => {
                let ka = match ka.texture_impl {
                    TextureImpl::OpenGL { texture } => texture,
                    #[cfg(windows)]
                    TextureImpl::Direct3D {  } => return None,
                    TextureImpl::Null {  } => return None,
                };

                let kd = match kd.texture_impl {
                    TextureImpl::OpenGL { texture } => texture,
                    #[cfg(windows)]
                    TextureImpl::Direct3D {  } => return None,
                    TextureImpl::Null {  } => return None,
                };

                let (buffer, slice) = match mesh.mesh_impl {
                    MeshImpl::OpenGL { buffer, slice } => (buffer, slice),
                    #[cfg(windows)]
                    MeshImpl::Direct3D {  } => return None,
                    MeshImpl::Null => return None,
                };

                let fragment = amethyst_renderer::Fragment {
                    transform: transform,
                    buffer: buffer,
                    slice: slice,
                    ka: ka,
                    kd: kd,
                };
                let fragment_impl = FragmentImpl::OpenGL {
                    fragment: fragment,
                };
                Some(Fragment {
                    fragment_impl: fragment_impl,
                })
            },
            #[cfg(windows)]
            FactoryImpl::Direct3D {  } => {
                unimplemented!();
            },
            FactoryImpl::Null => None,
        }
    }
}
