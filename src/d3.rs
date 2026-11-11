//! # 3D Rendering, Retained GPU Meshes, and Camera Builder
//!
//! Provides safe RAII wrappers around Raylib's 3D drawing mode ([`draw_3d`], [`Render3DContext`]),
//! OpenGL matrix stack operations (`push_matrix`, `mult_matrix`), chunked immediate-mode triangle
//! streaming (`draw_trimesh_buffered`), and retained VAO/VBO [`GpuMesh`] buffers.

use crate::*;

/// Fluent builder for constructing a [`Camera3D`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera3DBuilder {
    camera: Camera3D,
}

impl Default for Camera3DBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Camera3DBuilder {
    /// Creates a new `Camera3DBuilder` with sensible defaults.
    pub fn new() -> Self {
        Self {
            camera: Camera3D {
                position: Vector3::new(0.0, 10.0, 10.0),
                target: Vector3::zero(),
                up: Vector3::new(0.0, 1.0, 0.0),
                fovy: 45.0,
                projection: CameraProjection::CAMERA_PERSPECTIVE as i32,
            },
        }
    }

    /// Sets the camera position in 3D world space.
    pub fn position(mut self, pos: Vector3) -> Self {
        self.camera.position = pos;
        self
    }

    /// Sets the target point the camera is looking at.
    pub fn target(mut self, target: Vector3) -> Self {
        self.camera.target = target;
        self
    }

    /// Sets the camera's up vector (e.g. `(0.0, 1.0, 0.0)` for Y-up).
    pub fn up(mut self, up: Vector3) -> Self {
        self.camera.up = up;
        self
    }

    /// Sets the camera field of view in the Y axis in degrees (or ortho size).
    pub fn fovy(mut self, fovy: f32) -> Self {
        self.camera.fovy = fovy;
        self
    }

    /// Sets the camera projection type (e.g. [`CameraProjection::CAMERA_PERSPECTIVE`] or [`CameraProjection::CAMERA_ORTHOGRAPHIC`]).
    pub fn projection(mut self, projection: CameraProjection) -> Self {
        self.camera.projection = projection as i32;
        self
    }

    /// Builds and returns the configured [`Camera3D`].
    pub fn build(self) -> Camera3D {
        self.camera
    }
}

/// Safe drawing context active inside a [`draw_3d`] closure.
pub struct Render3DContext<'a> {
    pub(crate) _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> Render3DContext<'a> {
    /// Executes a closure with an isolated OpenGL matrix transformation stack.
    ///
    /// Automatically pushes a new matrix via `rlPushMatrix` and restores it via `rlPopMatrix`
    /// when the closure terminates, ensuring matrix state is preserved even in early returns or panics.
    pub fn push_matrix<F: FnOnce(&mut Render3DContext)>(&mut self, f: F) {
        struct MatrixGuard;
        impl Drop for MatrixGuard {
            fn drop(&mut self) {
                unsafe {
                    rlgl::rlPopMatrix();
                }
            }
        }

        unsafe {
            rlgl::rlPushMatrix();
        }
        let _guard = MatrixGuard;
        f(self);
    }

    /// Rotates the current coordinate system by `angle_deg` degrees around `axis`.
    pub fn rotate(&mut self, angle_deg: f32, axis: Vector3) {
        unsafe {
            rlgl::rlRotatef(angle_deg, axis.x, axis.y, axis.z);
        }
    }

    /// Translates the current coordinate system by `translation`.
    pub fn translate(&mut self, translation: Vector3) {
        unsafe {
            rlgl::rlTranslatef(translation.x, translation.y, translation.z);
        }
    }

    /// Scales the current coordinate system by `scale`.
    pub fn scale(&mut self, scale: Vector3) {
        unsafe {
            rlgl::rlScalef(scale.x, scale.y, scale.z);
        }
    }

    /// Multiplies the current OpenGL matrix by the given 4x4 [`Matrix`] without requiring `unsafe` pointer casts.
    pub fn mult_matrix(&mut self, matrix: &Matrix) {
        unsafe {
            rlgl::rlMultMatrixf(matrix as *const Matrix as *const f32);
        }
    }

    /// Draws a retained GPU-resident [`GpuMesh`] at the current matrix transform.
    pub fn draw_mesh(&self, mesh: &GpuMesh) {
        self.draw_mesh_transformed(mesh, Matrix::identity());
    }

    /// Draws a retained GPU-resident [`GpuMesh`] with an explicit model [`Matrix`] transform.
    pub fn draw_mesh_transformed(&self, mesh: &GpuMesh, transform: Matrix) {
        mesh.draw(transform);
    }

    /// Draws a solid 3D cube.
    pub fn draw_cube(&self, position: Vector3, width: f32, height: f32, length: f32, color: Color) {
        unsafe {
            DrawCube(position, width, height, length, color);
        }
    }

    /// Draws a wireframe 3D cube.
    pub fn draw_cube_wires(
        &self,
        position: Vector3,
        width: f32,
        height: f32,
        length: f32,
        color: Color,
    ) {
        unsafe {
            DrawCubeWires(position, width, height, length, color);
        }
    }

    /// Draws a solid 3D sphere.
    pub fn draw_sphere(&self, center: Vector3, radius: f32, color: Color) {
        unsafe {
            DrawSphere(center, radius, color);
        }
    }

    /// Draws a wireframe 3D sphere.
    pub fn draw_sphere_wires(
        &self,
        center: Vector3,
        radius: f32,
        rings: i32,
        slices: i32,
        color: Color,
    ) {
        unsafe {
            DrawSphereWires(center, radius, rings, slices, color);
        }
    }

    /// Draws a 3D cylinder.
    pub fn draw_cylinder(
        &self,
        position: Vector3,
        radius_top: f32,
        radius_bottom: f32,
        height: f32,
        sides: i32,
        color: Color,
    ) {
        unsafe {
            DrawCylinder(position, radius_top, radius_bottom, height, sides, color);
        }
    }

    /// Draws a wireframe 3D cylinder.
    pub fn draw_cylinder_wires(
        &self,
        position: Vector3,
        radius_top: f32,
        radius_bottom: f32,
        height: f32,
        sides: i32,
        color: Color,
    ) {
        unsafe {
            DrawCylinderWires(position, radius_top, radius_bottom, height, sides, color);
        }
    }

    /// Draws a 3D ground plane.
    pub fn draw_plane(&self, center: Vector3, size: Vector2, color: Color) {
        unsafe {
            DrawPlane(center, size, color);
        }
    }

    /// Draws a 3D line between `start` and `end`.
    pub fn draw_line_3d(&self, start: Vector3, end: Vector3, color: Color) {
        unsafe {
            DrawLine3D(start, end, color);
        }
    }

    /// Draws a 3D grid centered at origin.
    pub fn draw_grid(&self, slices: i32, spacing: f32) {
        unsafe {
            DrawGrid(slices, spacing);
        }
    }

    /// Draws a triangle mesh safely chunking draw calls to prevent `rlgl` vertex buffer overflows.
    ///
    /// Raylib's internal batch buffer defaults to 8,192 vertices. Pushing large procedural meshes
    /// without chunking and checking batch limits causes vertices to be dropped silently.
    /// This method automatically batches vertices into safe slices divisible by 3 (4,095 vertices max per chunk).
    pub fn draw_trimesh_buffered(&self, vertices: &[Vertex3D]) {
        const MAX_CHUNK_VERTICES: usize = 4095;
        for chunk in vertices.chunks(MAX_CHUNK_VERTICES) {
            unsafe {
                rlgl::rlCheckRenderBatchLimit(chunk.len() as i32);
                rlgl::rlBegin(rlgl::RL_TRIANGLES as i32);
                for v in chunk {
                    rlgl::rlColor4ub(v.color.r, v.color.g, v.color.b, v.color.a);
                    if let Some(norm) = v.normal {
                        rlgl::rlNormal3f(norm.x, norm.y, norm.z);
                    }
                    if let Some(uv) = v.uv {
                        rlgl::rlTexCoord2f(uv.x, uv.y);
                    }
                    rlgl::rlVertex3f(v.position.x, v.position.y, v.position.z);
                }
                rlgl::rlEnd();
            }
        }
    }

    /// Convenience method to draw flat colored triangles from position and color slices.
    pub fn draw_triangles(&self, positions: &[Vector3], colors: &[Color]) {
        let count = positions.len().min(colors.len());
        let vertices: Vec<Vertex3D> = (0..count)
            .map(|i| Vertex3D::new(positions[i], colors[i]))
            .collect();
        self.draw_trimesh_buffered(&vertices);
    }

    /// Draws a 3D billboard texture facing the camera.
    pub fn draw_billboard(
        &self,
        texture: Texture2D,
        position: Vector3,
        size: f32,
        tint: Color,
        camera: &Camera3D,
    ) {
        draw_billboard(*camera, texture, position, size, tint);
    }
}

/// A retained GPU-resident 3D mesh backed by OpenGL VAO and VBO buffers.
///
/// Unlike immediate-mode [`Render3DContext::draw_trimesh_buffered`], `GpuMesh` uploads vertex and
/// optional index buffers to GPU VRAM once and renders the entire mesh in a single draw call.
/// It can be constructed safely before window initialization (uploading lazily on first draw)
/// and automatically releases its GPU buffers when dropped.
pub struct GpuMesh {
    positions: Vec<f32>,
    normals: Vec<f32>,
    texcoords: Vec<f32>,
    colors: Vec<u8>,
    indices: Option<Vec<u16>>,
    gpu_handle: std::cell::RefCell<Option<(Mesh, Material)>>,
    gpu_capacity: std::cell::Cell<usize>,
    dirty: std::cell::Cell<bool>,
}

impl GpuMesh {
    /// Creates a new unindexed [`GpuMesh`] from a slice of [`Vertex3D`]s (3 vertices per triangle).
    pub fn from_vertices(vertices: &[Vertex3D]) -> Self {
        let mut mesh = Self {
            positions: Vec::with_capacity(vertices.len() * 3),
            normals: Vec::with_capacity(vertices.len() * 3),
            texcoords: Vec::with_capacity(vertices.len() * 2),
            colors: Vec::with_capacity(vertices.len() * 4),
            indices: None,
            gpu_handle: std::cell::RefCell::new(None),
            gpu_capacity: std::cell::Cell::new(0),
            dirty: std::cell::Cell::new(false),
        };
        mesh.pack_vertices(vertices);
        mesh.ensure_uploaded();
        mesh
    }

    /// Creates a new indexed [`GpuMesh`] from unique [`Vertex3D`]s and triangle indices (`3` indices per triangle).
    pub fn from_indexed(vertices: &[Vertex3D], indices: &[u16]) -> Self {
        let mut mesh = Self {
            positions: Vec::with_capacity(vertices.len() * 3),
            normals: Vec::with_capacity(vertices.len() * 3),
            texcoords: Vec::with_capacity(vertices.len() * 2),
            colors: Vec::with_capacity(vertices.len() * 4),
            indices: Some(indices.to_vec()),
            gpu_handle: std::cell::RefCell::new(None),
            gpu_capacity: std::cell::Cell::new(0),
            dirty: std::cell::Cell::new(false),
        };
        mesh.pack_vertices(vertices);
        mesh.ensure_uploaded();
        mesh
    }

    fn pack_vertices(&mut self, vertices: &[Vertex3D]) {
        self.positions.clear();
        self.normals.clear();
        self.texcoords.clear();
        self.colors.clear();

        for v in vertices {
            self.positions.push(v.position.x);
            self.positions.push(v.position.y);
            self.positions.push(v.position.z);

            let norm = v.normal.unwrap_or(Vector3::new(0.0, 1.0, 0.0));
            self.normals.push(norm.x);
            self.normals.push(norm.y);
            self.normals.push(norm.z);

            let uv = v.uv.unwrap_or(Vector2::zero());
            self.texcoords.push(uv.x);
            self.texcoords.push(uv.y);

            self.colors.push(v.color.r);
            self.colors.push(v.color.g);
            self.colors.push(v.color.b);
            self.colors.push(v.color.a);
        }
    }

    /// Updates the vertex attributes of the mesh dynamically.
    ///
    /// If the mesh has already been uploaded to the GPU and the new vertex count fits within
    /// the allocated GPU VBO capacity, its buffers are updated in-place via `UpdateMeshBuffer`
    /// without destroying or recreating the VAO. If the vertex count exceeds the current GPU
    /// capacity, the buffer is reallocated with 1.5x headroom to prevent VAO thrashing.
    pub fn update_vertices(&mut self, vertices: &[Vertex3D]) {
        let new_count = vertices.len();
        self.pack_vertices(vertices);

        if new_count == 0 {
            if let Some((ref mut rl_mesh, _)) = *self.gpu_handle.borrow_mut() {
                rl_mesh.vertexCount = 0;
                rl_mesh.triangleCount = 0;
            }
            self.dirty.set(false);
            return;
        }

        let current_cap = self.gpu_capacity.get();
        if !self.is_indexed() && self.gpu_handle.borrow().is_some() && new_count <= current_cap {
            if let Some((ref mut rl_mesh, _)) = *self.gpu_handle.borrow_mut() {
                rl_mesh.vertexCount = new_count as i32;
                rl_mesh.triangleCount = (new_count / 3) as i32;
            }
            self.dirty.set(true);
            if is_window_ready() {
                self.sync_dirty_buffers();
            }
        } else {
            self.unload_gpu();
            if is_window_ready() {
                let raw = (new_count * 3 / 2).max(new_count + 3072);
                let grown_cap = raw.div_ceil(3) * 3;
                self.upload_with_capacity(grown_cap);
            }
        }
    }

    /// Returns the number of vertices in this mesh.
    pub fn vertex_count(&self) -> usize {
        self.positions.len() / 3
    }

    /// Returns the number of triangles in this mesh.
    pub fn triangle_count(&self) -> usize {
        match &self.indices {
            Some(idx) => idx.len() / 3,
            None => self.vertex_count() / 3,
        }
    }

    /// Returns `true` if the mesh uses an index buffer.
    pub fn is_indexed(&self) -> bool {
        self.indices.is_some()
    }

    /// Returns `true` if the mesh has been uploaded to GPU VRAM.
    pub fn is_uploaded(&self) -> bool {
        self.gpu_handle.borrow().is_some()
    }

    fn upload_with_capacity(&mut self, capacity_vertices: usize) {
        let active_verts = self.vertex_count();
        let active_tris = self.triangle_count();
        if !is_window_ready() || active_verts == 0 {
            return;
        }
        let alloc_verts = capacity_vertices.max(active_verts);
        if alloc_verts > active_verts {
            self.positions.resize(alloc_verts * 3, 0.0);
            self.normals.resize(alloc_verts * 3, 0.0);
            self.texcoords.resize(alloc_verts * 2, 0.0);
            self.colors.resize(alloc_verts * 4, 0);
        }

        let mut rl_mesh = Mesh {
            vertexCount: alloc_verts as i32,
            triangleCount: (alloc_verts / 3) as i32,
            vertices: self.positions.as_ptr() as *mut f32,
            texcoords: self.texcoords.as_ptr() as *mut f32,
            normals: self.normals.as_ptr() as *mut f32,
            colors: self.colors.as_ptr() as *mut u8,
            ..Default::default()
        };
        if let Some(ref idx) = self.indices {
            rl_mesh.indices = idx.as_ptr() as *mut u16;
        }

        unsafe {
            UploadMesh(&mut rl_mesh, true);
            rl_mesh.vertices = std::ptr::null_mut();
            rl_mesh.texcoords = std::ptr::null_mut();
            rl_mesh.normals = std::ptr::null_mut();
            rl_mesh.colors = std::ptr::null_mut();
            rl_mesh.indices = std::ptr::null_mut();
            rl_mesh.vertexCount = active_verts as i32;
            rl_mesh.triangleCount = active_tris as i32;

            let material = LoadMaterialDefault();
            *self.gpu_handle.borrow_mut() = Some((rl_mesh, material));
            self.gpu_capacity.set(alloc_verts);
            self.dirty.set(false);
        }

        if alloc_verts > active_verts {
            self.positions.truncate(active_verts * 3);
            self.normals.truncate(active_verts * 3);
            self.texcoords.truncate(active_verts * 2);
            self.colors.truncate(active_verts * 4);
        }
    }

    fn ensure_uploaded(&self) {
        if !is_window_ready() || self.vertex_count() == 0 {
            return;
        }
        if self.gpu_handle.borrow().is_some() {
            if self.dirty.get() {
                self.sync_dirty_buffers();
            }
            return;
        }

        let mut rl_mesh = Mesh {
            vertexCount: self.vertex_count() as i32,
            triangleCount: self.triangle_count() as i32,
            vertices: self.positions.as_ptr() as *mut f32,
            texcoords: self.texcoords.as_ptr() as *mut f32,
            normals: self.normals.as_ptr() as *mut f32,
            colors: self.colors.as_ptr() as *mut u8,
            ..Default::default()
        };
        if let Some(ref idx) = self.indices {
            rl_mesh.indices = idx.as_ptr() as *mut u16;
        }

        unsafe {
            UploadMesh(&mut rl_mesh, true);
            rl_mesh.vertices = std::ptr::null_mut();
            rl_mesh.texcoords = std::ptr::null_mut();
            rl_mesh.normals = std::ptr::null_mut();
            rl_mesh.colors = std::ptr::null_mut();
            rl_mesh.indices = std::ptr::null_mut();

            let material = LoadMaterialDefault();
            *self.gpu_handle.borrow_mut() = Some((rl_mesh, material));
            self.gpu_capacity.set(self.vertex_count());
            self.dirty.set(false);
        }
    }

    fn sync_dirty_buffers(&self) {
        if let Some((rl_mesh, _)) = *self.gpu_handle.borrow() {
            unsafe {
                UpdateMeshBuffer(
                    rl_mesh,
                    0,
                    self.positions.as_ptr() as *const std::os::raw::c_void,
                    (self.positions.len() * std::mem::size_of::<f32>()) as i32,
                    0,
                );
                UpdateMeshBuffer(
                    rl_mesh,
                    1,
                    self.texcoords.as_ptr() as *const std::os::raw::c_void,
                    (self.texcoords.len() * std::mem::size_of::<f32>()) as i32,
                    0,
                );
                UpdateMeshBuffer(
                    rl_mesh,
                    2,
                    self.normals.as_ptr() as *const std::os::raw::c_void,
                    (self.normals.len() * std::mem::size_of::<f32>()) as i32,
                    0,
                );
                UpdateMeshBuffer(
                    rl_mesh,
                    3,
                    self.colors.as_ptr() as *const std::os::raw::c_void,
                    self.colors.len() as i32,
                    0,
                );
            }
            self.dirty.set(false);
        }
    }

    fn unload_gpu(&self) {
        if let Some((rl_mesh, _)) = self.gpu_handle.borrow_mut().take()
            && is_window_ready()
        {
            unsafe {
                UnloadMesh(rl_mesh);
            }
        }
        self.gpu_capacity.set(0);
    }

    pub(crate) fn draw(&self, transform: Matrix) {
        if !is_window_ready() || self.vertex_count() == 0 {
            return;
        }
        self.ensure_uploaded();
        if let Some((rl_mesh, material)) = *self.gpu_handle.borrow() {
            unsafe {
                DrawMesh(rl_mesh, material, transform);
            }
        }
    }
}

impl Drop for GpuMesh {
    fn drop(&mut self) {
        self.unload_gpu();
    }
}

/// A 3D vertex with position, color, and optional normal and texture coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vertex3D {
    /// 3D position vector.
    pub position: Vector3,
    /// Vertex color.
    pub color: Color,
    /// Optional surface normal vector.
    pub normal: Option<Vector3>,
    /// Optional UV texture coordinates.
    pub uv: Option<Vector2>,
}

impl Vertex3D {
    /// Creates a new vertex with position and color.
    pub fn new(position: Vector3, color: Color) -> Self {
        Self {
            position,
            color,
            normal: None,
            uv: None,
        }
    }

    /// Sets the normal vector for this vertex.
    pub fn with_normal(mut self, normal: Vector3) -> Self {
        self.normal = Some(normal);
        self
    }

    /// Sets the UV texture coordinates for this vertex.
    pub fn with_uv(mut self, uv: Vector2) -> Self {
        self.uv = Some(uv);
        self
    }
}

/// Executes a 3D rendering pass with the specified [`Camera3D`].
///
/// Sets up the 3D projection mode and guarantees cleanup via an RAII drop guard,
/// eliminating manual `unsafe { BeginMode3D(...) }` and `EndMode3D()` boilerplate.
pub fn draw_3d<F: FnOnce(&mut Render3DContext)>(camera: Camera3D, f: F) {
    struct Mode3DGuard;
    impl Drop for Mode3DGuard {
        fn drop(&mut self) {
            unsafe {
                EndMode3D();
            }
        }
    }

    unsafe {
        BeginMode3D(camera);
    }
    let _guard = Mode3DGuard;
    let mut ctx = Render3DContext {
        _marker: std::marker::PhantomData,
    };
    f(&mut ctx);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_mesh_unindexed_and_indexed() {
        let verts = [
            Vertex3D::new(Vector3::new(0.0, 0.0, 0.0), RED),
            Vertex3D::new(Vector3::new(1.0, 0.0, 0.0), GREEN),
            Vertex3D::new(Vector3::new(1.0, 0.0, 1.0), BLUE),
            Vertex3D::new(Vector3::new(0.0, 0.0, 1.0), YELLOW),
        ];
        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];

        let indexed_mesh = GpuMesh::from_indexed(&verts, &indices);
        assert_eq!(indexed_mesh.vertex_count(), 4);
        assert_eq!(indexed_mesh.triangle_count(), 2);
        assert!(indexed_mesh.is_indexed());
        assert!(!indexed_mesh.is_uploaded());

        let mut unindexed_mesh = GpuMesh::from_vertices(&verts[0..3]);
        assert_eq!(unindexed_mesh.vertex_count(), 3);
        assert_eq!(unindexed_mesh.triangle_count(), 1);
        assert!(!unindexed_mesh.is_indexed());

        let updated_verts = [
            Vertex3D::new(Vector3::new(0.0, 2.0, 0.0), WHITE),
            Vertex3D::new(Vector3::new(1.0, 2.0, 0.0), WHITE),
            Vertex3D::new(Vector3::new(1.0, 2.0, 1.0), WHITE),
        ];
        unindexed_mesh.update_vertices(&updated_verts);
        assert_eq!(unindexed_mesh.vertex_count(), 3);
        assert_eq!(unindexed_mesh.positions[1], 2.0);
    }
}
