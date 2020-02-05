
pub use self::types::*;
pub use self::enumerations::*;
pub use self::functions::*;

use std::os::raw;

pub struct FnPtr {
    ptr: *const raw::c_void,
    is_loaded: bool
}

impl FnPtr {
    pub fn empty() -> FnPtr {
        FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false }
    }

    pub fn load<F>(&mut self, loadfn: &mut F, name: &'static str) where F: FnMut(&'static str) -> *const raw::c_void {
        let loaded = loadfn(name);
        if !loaded.is_null() {
            self.ptr = loaded;
            self.is_loaded = true;
        } else {
            self.ptr = FnPtr::not_initialized as *const raw::c_void;
            self.is_loaded = false;
        };
    }

    pub fn aliased(&mut self, other: &FnPtr) {
        if !self.is_loaded && other.is_loaded {
            self.ptr = other.ptr;
            self.is_loaded = other.is_loaded;
        }
    }

    #[inline(never)]
    fn not_initialized() -> ! { panic!("gl: function not initialized") }
}

pub mod types {
#![allow(dead_code, non_snake_case, non_camel_case_types)]

use std::os::raw;

pub type GLvoid = raw::c_void;

pub type GLbyte = raw::c_char;
pub type GLubyte = raw::c_uchar;
pub type GLchar = raw::c_char;
pub type GLboolean = raw::c_uchar;

pub type GLshort = raw::c_short;
pub type GLushort = raw::c_ushort;

pub type GLint = raw::c_int;
pub type GLuint = raw::c_uint;
pub type GLint64 = i64;
pub type GLuint64 = u64;

pub type GLintptr = isize;
pub type GLsizeiptr = isize;
pub type GLintptrARB = isize;
pub type GLsizeiptrARB = isize;
pub type GLint64EXT = i64;
pub type GLuint64EXT = u64;

pub type GLsizei = GLint;
pub type GLclampx = raw::c_int;
pub type GLfixed = GLint;
pub type GLhalf = raw::c_ushort;
pub type GLhalfNV = raw::c_ushort;
pub type GLhalfARB = raw::c_ushort;

pub type GLenum = raw::c_uint;
pub type GLbitfield = raw::c_uint;

pub type GLfloat = raw::c_float;
pub type GLdouble = raw::c_double;
pub type GLclampf = raw::c_float;
pub type GLclampd = raw::c_double;

pub type GLcharARB = raw::c_char;

#[cfg(target_os = "macos")]
pub type GLhandleARB = *const raw::c_void;
#[cfg(not(target_os = "macos"))]
pub type GLhandleARB = raw::c_uint;

pub enum __GLsync {}

pub type GLsync = *const __GLsync;

pub enum _cl_context {}

pub enum _cl_event {}

pub type GLvdpauSurfaceNV = GLintptr;
pub type GLeglClientBufferEXT = *const raw::c_void;
pub type GLeglImageOES = *const raw::c_void;


pub type GLDEBUGPROC = extern "system" fn (
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut raw::c_void,
);
pub type GLDEBUGPROCARB = extern "system" fn (
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut raw::c_void,
);
pub type GLDEBUGPROCKHR = extern "system" fn (
    source: GLenum,
    type_: GLenum,
    id: GLuint,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut GLvoid,
);
pub type GLDEBUGPROCAMD = extern "system" fn (
    id: GLuint,
    category: GLenum,
    severity: GLenum,
    length: GLsizei,
    message: *const GLchar,
    userParam: *mut GLvoid,
);
pub type GLVULKANPROCNV = extern "system" fn ();
}

pub mod enumerations {
    #![allow(dead_code, non_upper_case_globals, unused_imports)]

    use std;
    use super::types::*;

    pub const GL_2D: std::os::raw::c_uint = 0x0600;
    pub const GL_2_BYTES: std::os::raw::c_uint = 0x1407;
    pub const GL_3D: std::os::raw::c_uint = 0x0601;
    pub const GL_3D_COLOR: std::os::raw::c_uint = 0x0602;
    pub const GL_3D_COLOR_TEXTURE: std::os::raw::c_uint = 0x0603;
    pub const GL_3_BYTES: std::os::raw::c_uint = 0x1408;
    pub const GL_4D_COLOR_TEXTURE: std::os::raw::c_uint = 0x0604;
    pub const GL_4_BYTES: std::os::raw::c_uint = 0x1409;
    pub const GL_ACCUM: std::os::raw::c_uint = 0x0100;
    pub const GL_ACCUM_ALPHA_BITS: std::os::raw::c_uint = 0x0D5B;
    pub const GL_ACCUM_BLUE_BITS: std::os::raw::c_uint = 0x0D5A;
    pub const GL_ACCUM_BUFFER_BIT: std::os::raw::c_uint = 0x00000200;
    pub const GL_ACCUM_CLEAR_VALUE: std::os::raw::c_uint = 0x0B80;
    pub const GL_ACCUM_GREEN_BITS: std::os::raw::c_uint = 0x0D59;
    pub const GL_ACCUM_RED_BITS: std::os::raw::c_uint = 0x0D58;
    pub const GL_ADD: std::os::raw::c_uint = 0x0104;
    pub const GL_ALL_ATTRIB_BITS: std::os::raw::c_uint = 0xFFFFFFFF;
    pub const GL_ALPHA: std::os::raw::c_uint = 0x1906;
    pub const GL_ALPHA_BIAS: std::os::raw::c_uint = 0x0D1D;
    pub const GL_ALPHA_BITS: std::os::raw::c_uint = 0x0D55;
    pub const GL_ALPHA_SCALE: std::os::raw::c_uint = 0x0D1C;
    pub const GL_ALPHA_TEST: std::os::raw::c_uint = 0x0BC0;
    pub const GL_ALPHA_TEST_FUNC: std::os::raw::c_uint = 0x0BC1;
    pub const GL_ALPHA_TEST_REF: std::os::raw::c_uint = 0x0BC2;
    pub const GL_ALWAYS: std::os::raw::c_uint = 0x0207;
    pub const GL_AMBIENT: std::os::raw::c_uint = 0x1200;
    pub const GL_AMBIENT_AND_DIFFUSE: std::os::raw::c_uint = 0x1602;
    pub const GL_AND: std::os::raw::c_uint = 0x1501;
    pub const GL_AND_INVERTED: std::os::raw::c_uint = 0x1504;
    pub const GL_AND_REVERSE: std::os::raw::c_uint = 0x1502;
    pub const GL_ATTRIB_STACK_DEPTH: std::os::raw::c_uint = 0x0BB0;
    pub const GL_AUTO_NORMAL: std::os::raw::c_uint = 0x0D80;
    pub const GL_AUX0: std::os::raw::c_uint = 0x0409;
    pub const GL_AUX1: std::os::raw::c_uint = 0x040A;
    pub const GL_AUX2: std::os::raw::c_uint = 0x040B;
    pub const GL_AUX3: std::os::raw::c_uint = 0x040C;
    pub const GL_AUX_BUFFERS: std::os::raw::c_uint = 0x0C00;
    pub const GL_BACK: std::os::raw::c_uint = 0x0405;
    pub const GL_BACK_LEFT: std::os::raw::c_uint = 0x0402;
    pub const GL_BACK_RIGHT: std::os::raw::c_uint = 0x0403;
    pub const GL_BITMAP: std::os::raw::c_uint = 0x1A00;
    pub const GL_BITMAP_TOKEN: std::os::raw::c_uint = 0x0704;
    pub const GL_BLEND: std::os::raw::c_uint = 0x0BE2;
    pub const GL_BLEND_DST: std::os::raw::c_uint = 0x0BE0;
    pub const GL_BLEND_SRC: std::os::raw::c_uint = 0x0BE1;
    pub const GL_BLUE: std::os::raw::c_uint = 0x1905;
    pub const GL_BLUE_BIAS: std::os::raw::c_uint = 0x0D1B;
    pub const GL_BLUE_BITS: std::os::raw::c_uint = 0x0D54;
    pub const GL_BLUE_SCALE: std::os::raw::c_uint = 0x0D1A;
    pub const GL_BYTE: std::os::raw::c_uint = 0x1400;
    pub const GL_CCW: std::os::raw::c_uint = 0x0901;
    pub const GL_CLAMP: std::os::raw::c_uint = 0x2900;
    pub const GL_CLEAR: std::os::raw::c_uint = 0x1500;
    pub const GL_CLIP_PLANE0: std::os::raw::c_uint = 0x3000;
    pub const GL_CLIP_PLANE1: std::os::raw::c_uint = 0x3001;
    pub const GL_CLIP_PLANE2: std::os::raw::c_uint = 0x3002;
    pub const GL_CLIP_PLANE3: std::os::raw::c_uint = 0x3003;
    pub const GL_CLIP_PLANE4: std::os::raw::c_uint = 0x3004;
    pub const GL_CLIP_PLANE5: std::os::raw::c_uint = 0x3005;
    pub const GL_COEFF: std::os::raw::c_uint = 0x0A00;
    pub const GL_COLOR: std::os::raw::c_uint = 0x1800;
    pub const GL_COLOR_BUFFER_BIT: std::os::raw::c_uint = 0x00004000;
    pub const GL_COLOR_CLEAR_VALUE: std::os::raw::c_uint = 0x0C22;
    pub const GL_COLOR_INDEX: std::os::raw::c_uint = 0x1900;
    pub const GL_COLOR_INDEXES: std::os::raw::c_uint = 0x1603;
    pub const GL_COLOR_MATERIAL: std::os::raw::c_uint = 0x0B57;
    pub const GL_COLOR_MATERIAL_FACE: std::os::raw::c_uint = 0x0B55;
    pub const GL_COLOR_MATERIAL_PARAMETER: std::os::raw::c_uint = 0x0B56;
    pub const GL_COLOR_WRITEMASK: std::os::raw::c_uint = 0x0C23;
    pub const GL_COMPILE: std::os::raw::c_uint = 0x1300;
    pub const GL_COMPILE_AND_EXECUTE: std::os::raw::c_uint = 0x1301;
    pub const GL_CONSTANT_ATTENUATION: std::os::raw::c_uint = 0x1207;
    pub const GL_COPY: std::os::raw::c_uint = 0x1503;
    pub const GL_COPY_INVERTED: std::os::raw::c_uint = 0x150C;
    pub const GL_COPY_PIXEL_TOKEN: std::os::raw::c_uint = 0x0706;
    pub const GL_CULL_FACE: std::os::raw::c_uint = 0x0B44;
    pub const GL_CULL_FACE_MODE: std::os::raw::c_uint = 0x0B45;
    pub const GL_CURRENT_BIT: std::os::raw::c_uint = 0x00000001;
    pub const GL_CURRENT_COLOR: std::os::raw::c_uint = 0x0B00;
    pub const GL_CURRENT_INDEX: std::os::raw::c_uint = 0x0B01;
    pub const GL_CURRENT_NORMAL: std::os::raw::c_uint = 0x0B02;
    pub const GL_CURRENT_RASTER_COLOR: std::os::raw::c_uint = 0x0B04;
    pub const GL_CURRENT_RASTER_DISTANCE: std::os::raw::c_uint = 0x0B09;
    pub const GL_CURRENT_RASTER_INDEX: std::os::raw::c_uint = 0x0B05;
    pub const GL_CURRENT_RASTER_POSITION: std::os::raw::c_uint = 0x0B07;
    pub const GL_CURRENT_RASTER_POSITION_VALID: std::os::raw::c_uint = 0x0B08;
    pub const GL_CURRENT_RASTER_TEXTURE_COORDS: std::os::raw::c_uint = 0x0B06;
    pub const GL_CURRENT_TEXTURE_COORDS: std::os::raw::c_uint = 0x0B03;
    pub const GL_CW: std::os::raw::c_uint = 0x0900;
    pub const GL_DECAL: std::os::raw::c_uint = 0x2101;
    pub const GL_DECR: std::os::raw::c_uint = 0x1E03;
    pub const GL_DEPTH: std::os::raw::c_uint = 0x1801;
    pub const GL_DEPTH_BIAS: std::os::raw::c_uint = 0x0D1F;
    pub const GL_DEPTH_BITS: std::os::raw::c_uint = 0x0D56;
    pub const GL_DEPTH_BUFFER_BIT: std::os::raw::c_uint = 0x00000100;
    pub const GL_DEPTH_CLEAR_VALUE: std::os::raw::c_uint = 0x0B73;
    pub const GL_DEPTH_COMPONENT: std::os::raw::c_uint = 0x1902;
    pub const GL_DEPTH_FUNC: std::os::raw::c_uint = 0x0B74;
    pub const GL_DEPTH_RANGE: std::os::raw::c_uint = 0x0B70;
    pub const GL_DEPTH_SCALE: std::os::raw::c_uint = 0x0D1E;
    pub const GL_DEPTH_TEST: std::os::raw::c_uint = 0x0B71;
    pub const GL_DEPTH_WRITEMASK: std::os::raw::c_uint = 0x0B72;
    pub const GL_DIFFUSE: std::os::raw::c_uint = 0x1201;
    pub const GL_DITHER: std::os::raw::c_uint = 0x0BD0;
    pub const GL_DOMAIN: std::os::raw::c_uint = 0x0A02;
    pub const GL_DONT_CARE: std::os::raw::c_uint = 0x1100;
    pub const GL_DOUBLEBUFFER: std::os::raw::c_uint = 0x0C32;
    pub const GL_DRAW_BUFFER: std::os::raw::c_uint = 0x0C01;
    pub const GL_DRAW_PIXEL_TOKEN: std::os::raw::c_uint = 0x0705;
    pub const GL_DST_ALPHA: std::os::raw::c_uint = 0x0304;
    pub const GL_DST_COLOR: std::os::raw::c_uint = 0x0306;
    pub const GL_EDGE_FLAG: std::os::raw::c_uint = 0x0B43;
    pub const GL_EMISSION: std::os::raw::c_uint = 0x1600;
    pub const GL_ENABLE_BIT: std::os::raw::c_uint = 0x00002000;
    pub const GL_EQUAL: std::os::raw::c_uint = 0x0202;
    pub const GL_EQUIV: std::os::raw::c_uint = 0x1509;
    pub const GL_EVAL_BIT: std::os::raw::c_uint = 0x00010000;
    pub const GL_EXP: std::os::raw::c_uint = 0x0800;
    pub const GL_EXP2: std::os::raw::c_uint = 0x0801;
    pub const GL_EXTENSIONS: std::os::raw::c_uint = 0x1F03;
    pub const GL_EYE_LINEAR: std::os::raw::c_uint = 0x2400;
    pub const GL_EYE_PLANE: std::os::raw::c_uint = 0x2502;
    pub const GL_FALSE: std::os::raw::c_uchar = 0;
    pub const GL_FASTEST: std::os::raw::c_uint = 0x1101;
    pub const GL_FEEDBACK: std::os::raw::c_uint = 0x1C01;
    pub const GL_FILL: std::os::raw::c_uint = 0x1B02;
    pub const GL_FLAT: std::os::raw::c_uint = 0x1D00;
    pub const GL_FLOAT: std::os::raw::c_uint = 0x1406;
    pub const GL_FOG: std::os::raw::c_uint = 0x0B60;
    pub const GL_FOG_BIT: std::os::raw::c_uint = 0x00000080;
    pub const GL_FOG_COLOR: std::os::raw::c_uint = 0x0B66;
    pub const GL_FOG_DENSITY: std::os::raw::c_uint = 0x0B62;
    pub const GL_FOG_END: std::os::raw::c_uint = 0x0B64;
    pub const GL_FOG_HINT: std::os::raw::c_uint = 0x0C54;
    pub const GL_FOG_INDEX: std::os::raw::c_uint = 0x0B61;
    pub const GL_FOG_MODE: std::os::raw::c_uint = 0x0B65;
    pub const GL_FOG_START: std::os::raw::c_uint = 0x0B63;
    pub const GL_FRONT: std::os::raw::c_uint = 0x0404;
    pub const GL_FRONT_AND_BACK: std::os::raw::c_uint = 0x0408;
    pub const GL_FRONT_FACE: std::os::raw::c_uint = 0x0B46;
    pub const GL_FRONT_LEFT: std::os::raw::c_uint = 0x0400;
    pub const GL_FRONT_RIGHT: std::os::raw::c_uint = 0x0401;
    pub const GL_GEQUAL: std::os::raw::c_uint = 0x0206;
    pub const GL_GREATER: std::os::raw::c_uint = 0x0204;
    pub const GL_GREEN: std::os::raw::c_uint = 0x1904;
    pub const GL_GREEN_BIAS: std::os::raw::c_uint = 0x0D19;
    pub const GL_GREEN_BITS: std::os::raw::c_uint = 0x0D53;
    pub const GL_GREEN_SCALE: std::os::raw::c_uint = 0x0D18;
    pub const GL_HINT_BIT: std::os::raw::c_uint = 0x00008000;
    pub const GL_INCR: std::os::raw::c_uint = 0x1E02;
    pub const GL_INDEX_BITS: std::os::raw::c_uint = 0x0D51;
    pub const GL_INDEX_CLEAR_VALUE: std::os::raw::c_uint = 0x0C20;
    pub const GL_INDEX_MODE: std::os::raw::c_uint = 0x0C30;
    pub const GL_INDEX_OFFSET: std::os::raw::c_uint = 0x0D13;
    pub const GL_INDEX_SHIFT: std::os::raw::c_uint = 0x0D12;
    pub const GL_INDEX_WRITEMASK: std::os::raw::c_uint = 0x0C21;
    pub const GL_INT: std::os::raw::c_uint = 0x1404;
    pub const GL_INVALID_ENUM: std::os::raw::c_uint = 0x0500;
    pub const GL_INVALID_OPERATION: std::os::raw::c_uint = 0x0502;
    pub const GL_INVALID_VALUE: std::os::raw::c_uint = 0x0501;
    pub const GL_INVERT: std::os::raw::c_uint = 0x150A;
    pub const GL_KEEP: std::os::raw::c_uint = 0x1E00;
    pub const GL_LEFT: std::os::raw::c_uint = 0x0406;
    pub const GL_LEQUAL: std::os::raw::c_uint = 0x0203;
    pub const GL_LESS: std::os::raw::c_uint = 0x0201;
    pub const GL_LIGHT0: std::os::raw::c_uint = 0x4000;
    pub const GL_LIGHT1: std::os::raw::c_uint = 0x4001;
    pub const GL_LIGHT2: std::os::raw::c_uint = 0x4002;
    pub const GL_LIGHT3: std::os::raw::c_uint = 0x4003;
    pub const GL_LIGHT4: std::os::raw::c_uint = 0x4004;
    pub const GL_LIGHT5: std::os::raw::c_uint = 0x4005;
    pub const GL_LIGHT6: std::os::raw::c_uint = 0x4006;
    pub const GL_LIGHT7: std::os::raw::c_uint = 0x4007;
    pub const GL_LIGHTING: std::os::raw::c_uint = 0x0B50;
    pub const GL_LIGHTING_BIT: std::os::raw::c_uint = 0x00000040;
    pub const GL_LIGHT_MODEL_AMBIENT: std::os::raw::c_uint = 0x0B53;
    pub const GL_LIGHT_MODEL_LOCAL_VIEWER: std::os::raw::c_uint = 0x0B51;
    pub const GL_LIGHT_MODEL_TWO_SIDE: std::os::raw::c_uint = 0x0B52;
    pub const GL_LINE: std::os::raw::c_uint = 0x1B01;
    pub const GL_LINEAR: std::os::raw::c_uint = 0x2601;
    pub const GL_LINEAR_ATTENUATION: std::os::raw::c_uint = 0x1208;
    pub const GL_LINEAR_MIPMAP_LINEAR: std::os::raw::c_uint = 0x2703;
    pub const GL_LINEAR_MIPMAP_NEAREST: std::os::raw::c_uint = 0x2701;
    pub const GL_LINES: std::os::raw::c_uint = 0x0001;
    pub const GL_LINE_BIT: std::os::raw::c_uint = 0x00000004;
    pub const GL_LINE_LOOP: std::os::raw::c_uint = 0x0002;
    pub const GL_LINE_RESET_TOKEN: std::os::raw::c_uint = 0x0707;
    pub const GL_LINE_SMOOTH: std::os::raw::c_uint = 0x0B20;
    pub const GL_LINE_SMOOTH_HINT: std::os::raw::c_uint = 0x0C52;
    pub const GL_LINE_STIPPLE: std::os::raw::c_uint = 0x0B24;
    pub const GL_LINE_STIPPLE_PATTERN: std::os::raw::c_uint = 0x0B25;
    pub const GL_LINE_STIPPLE_REPEAT: std::os::raw::c_uint = 0x0B26;
    pub const GL_LINE_STRIP: std::os::raw::c_uint = 0x0003;
    pub const GL_LINE_TOKEN: std::os::raw::c_uint = 0x0702;
    pub const GL_LINE_WIDTH: std::os::raw::c_uint = 0x0B21;
    pub const GL_LINE_WIDTH_GRANULARITY: std::os::raw::c_uint = 0x0B23;
    pub const GL_LINE_WIDTH_RANGE: std::os::raw::c_uint = 0x0B22;
    pub const GL_LIST_BASE: std::os::raw::c_uint = 0x0B32;
    pub const GL_LIST_BIT: std::os::raw::c_uint = 0x00020000;
    pub const GL_LIST_INDEX: std::os::raw::c_uint = 0x0B33;
    pub const GL_LIST_MODE: std::os::raw::c_uint = 0x0B30;
    pub const GL_LOAD: std::os::raw::c_uint = 0x0101;
    pub const GL_LOGIC_OP: std::os::raw::c_uint = 0x0BF1;
    pub const GL_LOGIC_OP_MODE: std::os::raw::c_uint = 0x0BF0;
    pub const GL_LUMINANCE: std::os::raw::c_uint = 0x1909;
    pub const GL_LUMINANCE_ALPHA: std::os::raw::c_uint = 0x190A;
    pub const GL_MAP1_COLOR_4: std::os::raw::c_uint = 0x0D90;
    pub const GL_MAP1_GRID_DOMAIN: std::os::raw::c_uint = 0x0DD0;
    pub const GL_MAP1_GRID_SEGMENTS: std::os::raw::c_uint = 0x0DD1;
    pub const GL_MAP1_INDEX: std::os::raw::c_uint = 0x0D91;
    pub const GL_MAP1_NORMAL: std::os::raw::c_uint = 0x0D92;
    pub const GL_MAP1_TEXTURE_COORD_1: std::os::raw::c_uint = 0x0D93;
    pub const GL_MAP1_TEXTURE_COORD_2: std::os::raw::c_uint = 0x0D94;
    pub const GL_MAP1_TEXTURE_COORD_3: std::os::raw::c_uint = 0x0D95;
    pub const GL_MAP1_TEXTURE_COORD_4: std::os::raw::c_uint = 0x0D96;
    pub const GL_MAP1_VERTEX_3: std::os::raw::c_uint = 0x0D97;
    pub const GL_MAP1_VERTEX_4: std::os::raw::c_uint = 0x0D98;
    pub const GL_MAP2_COLOR_4: std::os::raw::c_uint = 0x0DB0;
    pub const GL_MAP2_GRID_DOMAIN: std::os::raw::c_uint = 0x0DD2;
    pub const GL_MAP2_GRID_SEGMENTS: std::os::raw::c_uint = 0x0DD3;
    pub const GL_MAP2_INDEX: std::os::raw::c_uint = 0x0DB1;
    pub const GL_MAP2_NORMAL: std::os::raw::c_uint = 0x0DB2;
    pub const GL_MAP2_TEXTURE_COORD_1: std::os::raw::c_uint = 0x0DB3;
    pub const GL_MAP2_TEXTURE_COORD_2: std::os::raw::c_uint = 0x0DB4;
    pub const GL_MAP2_TEXTURE_COORD_3: std::os::raw::c_uint = 0x0DB5;
    pub const GL_MAP2_TEXTURE_COORD_4: std::os::raw::c_uint = 0x0DB6;
    pub const GL_MAP2_VERTEX_3: std::os::raw::c_uint = 0x0DB7;
    pub const GL_MAP2_VERTEX_4: std::os::raw::c_uint = 0x0DB8;
    pub const GL_MAP_COLOR: std::os::raw::c_uint = 0x0D10;
    pub const GL_MAP_STENCIL: std::os::raw::c_uint = 0x0D11;
    pub const GL_MATRIX_MODE: std::os::raw::c_uint = 0x0BA0;
    pub const GL_MAX_ATTRIB_STACK_DEPTH: std::os::raw::c_uint = 0x0D35;
    pub const GL_MAX_CLIP_PLANES: std::os::raw::c_uint = 0x0D32;
    pub const GL_MAX_EVAL_ORDER: std::os::raw::c_uint = 0x0D30;
    pub const GL_MAX_LIGHTS: std::os::raw::c_uint = 0x0D31;
    pub const GL_MAX_LIST_NESTING: std::os::raw::c_uint = 0x0B31;
    pub const GL_MAX_MODELVIEW_STACK_DEPTH: std::os::raw::c_uint = 0x0D36;
    pub const GL_MAX_NAME_STACK_DEPTH: std::os::raw::c_uint = 0x0D37;
    pub const GL_MAX_PIXEL_MAP_TABLE: std::os::raw::c_uint = 0x0D34;
    pub const GL_MAX_PROJECTION_STACK_DEPTH: std::os::raw::c_uint = 0x0D38;
    pub const GL_MAX_TEXTURE_SIZE: std::os::raw::c_uint = 0x0D33;
    pub const GL_MAX_TEXTURE_STACK_DEPTH: std::os::raw::c_uint = 0x0D39;
    pub const GL_MAX_VIEWPORT_DIMS: std::os::raw::c_uint = 0x0D3A;
    pub const GL_MODELVIEW: std::os::raw::c_uint = 0x1700;
    pub const GL_MODELVIEW_MATRIX: std::os::raw::c_uint = 0x0BA6;
    pub const GL_MODELVIEW_STACK_DEPTH: std::os::raw::c_uint = 0x0BA3;
    pub const GL_MODULATE: std::os::raw::c_uint = 0x2100;
    pub const GL_MULT: std::os::raw::c_uint = 0x0103;
    pub const GL_NAME_STACK_DEPTH: std::os::raw::c_uint = 0x0D70;
    pub const GL_NAND: std::os::raw::c_uint = 0x150E;
    pub const GL_NEAREST: std::os::raw::c_uint = 0x2600;
    pub const GL_NEAREST_MIPMAP_LINEAR: std::os::raw::c_uint = 0x2702;
    pub const GL_NEAREST_MIPMAP_NEAREST: std::os::raw::c_uint = 0x2700;
    pub const GL_NEVER: std::os::raw::c_uint = 0x0200;
    pub const GL_NICEST: std::os::raw::c_uint = 0x1102;
    pub const GL_NONE: std::os::raw::c_uint = 0;
    pub const GL_NOOP: std::os::raw::c_uint = 0x1505;
    pub const GL_NOR: std::os::raw::c_uint = 0x1508;
    pub const GL_NORMALIZE: std::os::raw::c_uint = 0x0BA1;
    pub const GL_NOTEQUAL: std::os::raw::c_uint = 0x0205;
    pub const GL_NO_ERROR: std::os::raw::c_uint = 0;
    pub const GL_OBJECT_LINEAR: std::os::raw::c_uint = 0x2401;
    pub const GL_OBJECT_PLANE: std::os::raw::c_uint = 0x2501;
    pub const GL_ONE: std::os::raw::c_uint = 1;
    pub const GL_ONE_MINUS_DST_ALPHA: std::os::raw::c_uint = 0x0305;
    pub const GL_ONE_MINUS_DST_COLOR: std::os::raw::c_uint = 0x0307;
    pub const GL_ONE_MINUS_SRC_ALPHA: std::os::raw::c_uint = 0x0303;
    pub const GL_ONE_MINUS_SRC_COLOR: std::os::raw::c_uint = 0x0301;
    pub const GL_OR: std::os::raw::c_uint = 0x1507;
    pub const GL_ORDER: std::os::raw::c_uint = 0x0A01;
    pub const GL_OR_INVERTED: std::os::raw::c_uint = 0x150D;
    pub const GL_OR_REVERSE: std::os::raw::c_uint = 0x150B;
    pub const GL_OUT_OF_MEMORY: std::os::raw::c_uint = 0x0505;
    pub const GL_PACK_ALIGNMENT: std::os::raw::c_uint = 0x0D05;
    pub const GL_PACK_LSB_FIRST: std::os::raw::c_uint = 0x0D01;
    pub const GL_PACK_ROW_LENGTH: std::os::raw::c_uint = 0x0D02;
    pub const GL_PACK_SKIP_PIXELS: std::os::raw::c_uint = 0x0D04;
    pub const GL_PACK_SKIP_ROWS: std::os::raw::c_uint = 0x0D03;
    pub const GL_PACK_SWAP_BYTES: std::os::raw::c_uint = 0x0D00;
    pub const GL_PASS_THROUGH_TOKEN: std::os::raw::c_uint = 0x0700;
    pub const GL_PERSPECTIVE_CORRECTION_HINT: std::os::raw::c_uint = 0x0C50;
    pub const GL_PIXEL_MAP_A_TO_A: std::os::raw::c_uint = 0x0C79;
    pub const GL_PIXEL_MAP_A_TO_A_SIZE: std::os::raw::c_uint = 0x0CB9;
    pub const GL_PIXEL_MAP_B_TO_B: std::os::raw::c_uint = 0x0C78;
    pub const GL_PIXEL_MAP_B_TO_B_SIZE: std::os::raw::c_uint = 0x0CB8;
    pub const GL_PIXEL_MAP_G_TO_G: std::os::raw::c_uint = 0x0C77;
    pub const GL_PIXEL_MAP_G_TO_G_SIZE: std::os::raw::c_uint = 0x0CB7;
    pub const GL_PIXEL_MAP_I_TO_A: std::os::raw::c_uint = 0x0C75;
    pub const GL_PIXEL_MAP_I_TO_A_SIZE: std::os::raw::c_uint = 0x0CB5;
    pub const GL_PIXEL_MAP_I_TO_B: std::os::raw::c_uint = 0x0C74;
    pub const GL_PIXEL_MAP_I_TO_B_SIZE: std::os::raw::c_uint = 0x0CB4;
    pub const GL_PIXEL_MAP_I_TO_G: std::os::raw::c_uint = 0x0C73;
    pub const GL_PIXEL_MAP_I_TO_G_SIZE: std::os::raw::c_uint = 0x0CB3;
    pub const GL_PIXEL_MAP_I_TO_I: std::os::raw::c_uint = 0x0C70;
    pub const GL_PIXEL_MAP_I_TO_I_SIZE: std::os::raw::c_uint = 0x0CB0;
    pub const GL_PIXEL_MAP_I_TO_R: std::os::raw::c_uint = 0x0C72;
    pub const GL_PIXEL_MAP_I_TO_R_SIZE: std::os::raw::c_uint = 0x0CB2;
    pub const GL_PIXEL_MAP_R_TO_R: std::os::raw::c_uint = 0x0C76;
    pub const GL_PIXEL_MAP_R_TO_R_SIZE: std::os::raw::c_uint = 0x0CB6;
    pub const GL_PIXEL_MAP_S_TO_S: std::os::raw::c_uint = 0x0C71;
    pub const GL_PIXEL_MAP_S_TO_S_SIZE: std::os::raw::c_uint = 0x0CB1;
    pub const GL_PIXEL_MODE_BIT: std::os::raw::c_uint = 0x00000020;
    pub const GL_POINT: std::os::raw::c_uint = 0x1B00;
    pub const GL_POINTS: std::os::raw::c_uint = 0x0000;
    pub const GL_POINT_BIT: std::os::raw::c_uint = 0x00000002;
    pub const GL_POINT_SIZE: std::os::raw::c_uint = 0x0B11;
    pub const GL_POINT_SIZE_GRANULARITY: std::os::raw::c_uint = 0x0B13;
    pub const GL_POINT_SIZE_RANGE: std::os::raw::c_uint = 0x0B12;
    pub const GL_POINT_SMOOTH: std::os::raw::c_uint = 0x0B10;
    pub const GL_POINT_SMOOTH_HINT: std::os::raw::c_uint = 0x0C51;
    pub const GL_POINT_TOKEN: std::os::raw::c_uint = 0x0701;
    pub const GL_POLYGON: std::os::raw::c_uint = 0x0009;
    pub const GL_POLYGON_BIT: std::os::raw::c_uint = 0x00000008;
    pub const GL_POLYGON_MODE: std::os::raw::c_uint = 0x0B40;
    pub const GL_POLYGON_SMOOTH: std::os::raw::c_uint = 0x0B41;
    pub const GL_POLYGON_SMOOTH_HINT: std::os::raw::c_uint = 0x0C53;
    pub const GL_POLYGON_STIPPLE: std::os::raw::c_uint = 0x0B42;
    pub const GL_POLYGON_STIPPLE_BIT: std::os::raw::c_uint = 0x00000010;
    pub const GL_POLYGON_TOKEN: std::os::raw::c_uint = 0x0703;
    pub const GL_POSITION: std::os::raw::c_uint = 0x1203;
    pub const GL_PROJECTION: std::os::raw::c_uint = 0x1701;
    pub const GL_PROJECTION_MATRIX: std::os::raw::c_uint = 0x0BA7;
    pub const GL_PROJECTION_STACK_DEPTH: std::os::raw::c_uint = 0x0BA4;
    pub const GL_Q: std::os::raw::c_uint = 0x2003;
    pub const GL_QUADRATIC_ATTENUATION: std::os::raw::c_uint = 0x1209;
    pub const GL_QUADS: std::os::raw::c_uint = 0x0007;
    pub const GL_QUAD_STRIP: std::os::raw::c_uint = 0x0008;
    pub const GL_R: std::os::raw::c_uint = 0x2002;
    pub const GL_READ_BUFFER: std::os::raw::c_uint = 0x0C02;
    pub const GL_RED: std::os::raw::c_uint = 0x1903;
    pub const GL_RED_BIAS: std::os::raw::c_uint = 0x0D15;
    pub const GL_RED_BITS: std::os::raw::c_uint = 0x0D52;
    pub const GL_RED_SCALE: std::os::raw::c_uint = 0x0D14;
    pub const GL_RENDER: std::os::raw::c_uint = 0x1C00;
    pub const GL_RENDERER: std::os::raw::c_uint = 0x1F01;
    pub const GL_RENDER_MODE: std::os::raw::c_uint = 0x0C40;
    pub const GL_REPEAT: std::os::raw::c_uint = 0x2901;
    pub const GL_REPLACE: std::os::raw::c_uint = 0x1E01;
    pub const GL_RETURN: std::os::raw::c_uint = 0x0102;
    pub const GL_RGB: std::os::raw::c_uint = 0x1907;
    pub const GL_RGBA: std::os::raw::c_uint = 0x1908;
    pub const GL_RGBA_MODE: std::os::raw::c_uint = 0x0C31;
    pub const GL_RIGHT: std::os::raw::c_uint = 0x0407;
    pub const GL_S: std::os::raw::c_uint = 0x2000;
    pub const GL_SCISSOR_BIT: std::os::raw::c_uint = 0x00080000;
    pub const GL_SCISSOR_BOX: std::os::raw::c_uint = 0x0C10;
    pub const GL_SCISSOR_TEST: std::os::raw::c_uint = 0x0C11;
    pub const GL_SELECT: std::os::raw::c_uint = 0x1C02;
    pub const GL_SET: std::os::raw::c_uint = 0x150F;
    pub const GL_SHADE_MODEL: std::os::raw::c_uint = 0x0B54;
    pub const GL_SHININESS: std::os::raw::c_uint = 0x1601;
    pub const GL_SHORT: std::os::raw::c_uint = 0x1402;
    pub const GL_SMOOTH: std::os::raw::c_uint = 0x1D01;
    pub const GL_SPECULAR: std::os::raw::c_uint = 0x1202;
    pub const GL_SPHERE_MAP: std::os::raw::c_uint = 0x2402;
    pub const GL_SPOT_CUTOFF: std::os::raw::c_uint = 0x1206;
    pub const GL_SPOT_DIRECTION: std::os::raw::c_uint = 0x1204;
    pub const GL_SPOT_EXPONENT: std::os::raw::c_uint = 0x1205;
    pub const GL_SRC_ALPHA: std::os::raw::c_uint = 0x0302;
    pub const GL_SRC_ALPHA_SATURATE: std::os::raw::c_uint = 0x0308;
    pub const GL_SRC_COLOR: std::os::raw::c_uint = 0x0300;
    pub const GL_STACK_OVERFLOW: std::os::raw::c_uint = 0x0503;
    pub const GL_STACK_UNDERFLOW: std::os::raw::c_uint = 0x0504;
    pub const GL_STENCIL: std::os::raw::c_uint = 0x1802;
    pub const GL_STENCIL_BITS: std::os::raw::c_uint = 0x0D57;
    pub const GL_STENCIL_BUFFER_BIT: std::os::raw::c_uint = 0x00000400;
    pub const GL_STENCIL_CLEAR_VALUE: std::os::raw::c_uint = 0x0B91;
    pub const GL_STENCIL_FAIL: std::os::raw::c_uint = 0x0B94;
    pub const GL_STENCIL_FUNC: std::os::raw::c_uint = 0x0B92;
    pub const GL_STENCIL_INDEX: std::os::raw::c_uint = 0x1901;
    pub const GL_STENCIL_PASS_DEPTH_FAIL: std::os::raw::c_uint = 0x0B95;
    pub const GL_STENCIL_PASS_DEPTH_PASS: std::os::raw::c_uint = 0x0B96;
    pub const GL_STENCIL_REF: std::os::raw::c_uint = 0x0B97;
    pub const GL_STENCIL_TEST: std::os::raw::c_uint = 0x0B90;
    pub const GL_STENCIL_VALUE_MASK: std::os::raw::c_uint = 0x0B93;
    pub const GL_STENCIL_WRITEMASK: std::os::raw::c_uint = 0x0B98;
    pub const GL_STEREO: std::os::raw::c_uint = 0x0C33;
    pub const GL_SUBPIXEL_BITS: std::os::raw::c_uint = 0x0D50;
    pub const GL_T: std::os::raw::c_uint = 0x2001;
    pub const GL_TEXTURE: std::os::raw::c_uint = 0x1702;
    pub const GL_TEXTURE_1D: std::os::raw::c_uint = 0x0DE0;
    pub const GL_TEXTURE_2D: std::os::raw::c_uint = 0x0DE1;
    pub const GL_TEXTURE_BIT: std::os::raw::c_uint = 0x00040000;
    pub const GL_TEXTURE_BORDER: std::os::raw::c_uint = 0x1005;
    pub const GL_TEXTURE_BORDER_COLOR: std::os::raw::c_uint = 0x1004;
    pub const GL_TEXTURE_COMPONENTS: std::os::raw::c_uint = 0x1003;
    pub const GL_TEXTURE_ENV: std::os::raw::c_uint = 0x2300;
    pub const GL_TEXTURE_ENV_COLOR: std::os::raw::c_uint = 0x2201;
    pub const GL_TEXTURE_ENV_MODE: std::os::raw::c_uint = 0x2200;
    pub const GL_TEXTURE_GEN_MODE: std::os::raw::c_uint = 0x2500;
    pub const GL_TEXTURE_GEN_Q: std::os::raw::c_uint = 0x0C63;
    pub const GL_TEXTURE_GEN_R: std::os::raw::c_uint = 0x0C62;
    pub const GL_TEXTURE_GEN_S: std::os::raw::c_uint = 0x0C60;
    pub const GL_TEXTURE_GEN_T: std::os::raw::c_uint = 0x0C61;
    pub const GL_TEXTURE_HEIGHT: std::os::raw::c_uint = 0x1001;
    pub const GL_TEXTURE_MAG_FILTER: std::os::raw::c_uint = 0x2800;
    pub const GL_TEXTURE_MATRIX: std::os::raw::c_uint = 0x0BA8;
    pub const GL_TEXTURE_MIN_FILTER: std::os::raw::c_uint = 0x2801;
    pub const GL_TEXTURE_STACK_DEPTH: std::os::raw::c_uint = 0x0BA5;
    pub const GL_TEXTURE_WIDTH: std::os::raw::c_uint = 0x1000;
    pub const GL_TEXTURE_WRAP_S: std::os::raw::c_uint = 0x2802;
    pub const GL_TEXTURE_WRAP_T: std::os::raw::c_uint = 0x2803;
    pub const GL_TRANSFORM_BIT: std::os::raw::c_uint = 0x00001000;
    pub const GL_TRIANGLES: std::os::raw::c_uint = 0x0004;
    pub const GL_TRIANGLE_FAN: std::os::raw::c_uint = 0x0006;
    pub const GL_TRIANGLE_STRIP: std::os::raw::c_uint = 0x0005;
    pub const GL_TRUE: std::os::raw::c_uchar = 1;
    pub const GL_UNPACK_ALIGNMENT: std::os::raw::c_uint = 0x0CF5;
    pub const GL_UNPACK_LSB_FIRST: std::os::raw::c_uint = 0x0CF1;
    pub const GL_UNPACK_ROW_LENGTH: std::os::raw::c_uint = 0x0CF2;
    pub const GL_UNPACK_SKIP_PIXELS: std::os::raw::c_uint = 0x0CF4;
    pub const GL_UNPACK_SKIP_ROWS: std::os::raw::c_uint = 0x0CF3;
    pub const GL_UNPACK_SWAP_BYTES: std::os::raw::c_uint = 0x0CF0;
    pub const GL_UNSIGNED_BYTE: std::os::raw::c_uint = 0x1401;
    pub const GL_UNSIGNED_INT: std::os::raw::c_uint = 0x1405;
    pub const GL_UNSIGNED_SHORT: std::os::raw::c_uint = 0x1403;
    pub const GL_VENDOR: std::os::raw::c_uint = 0x1F00;
    pub const GL_VERSION: std::os::raw::c_uint = 0x1F02;
    pub const GL_VIEWPORT: std::os::raw::c_uint = 0x0BA2;
    pub const GL_VIEWPORT_BIT: std::os::raw::c_uint = 0x00000800;
    pub const GL_XOR: std::os::raw::c_uint = 0x1506;
    pub const GL_ZERO: std::os::raw::c_uint = 0;
    pub const GL_ZOOM_X: std::os::raw::c_uint = 0x0D16;
    pub const GL_ZOOM_Y: std::os::raw::c_uint = 0x0D17;
}

pub mod functions {
    #![allow(non_snake_case, unused_variables, dead_code)]

    use std;
    use std::mem;
    use super::storage;
    use super::types::*;

     #[inline] pub unsafe fn Accum(op: GLenum, value: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLfloat) -> ()>(storage::Accum.ptr)(op, value) }
     #[inline] pub unsafe fn AlphaFunc(func: GLenum, ref_: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLfloat) -> ()>(storage::AlphaFunc.ptr)(func, ref_) }
     #[inline] pub unsafe fn Begin(mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::Begin.ptr)(mode) }
     #[inline] pub unsafe fn Bitmap(width: GLsizei, height: GLsizei, xorig: GLfloat, yorig: GLfloat, xmove: GLfloat, ymove: GLfloat, bitmap: *const GLubyte) -> () { mem::transmute::<_, extern "system" fn(GLsizei, GLsizei, GLfloat, GLfloat, GLfloat, GLfloat, *const GLubyte) -> ()>(storage::Bitmap.ptr)(width, height, xorig, yorig, xmove, ymove, bitmap) }
     #[inline] pub unsafe fn BlendFunc(sfactor: GLenum, dfactor: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(storage::BlendFunc.ptr)(sfactor, dfactor) }
     #[inline] pub unsafe fn CallList(list: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::CallList.ptr)(list) }
     #[inline] pub unsafe fn CallLists(n: GLsizei, type_: GLenum, lists: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLsizei, GLenum, *const std::os::raw::c_void) -> ()>(storage::CallLists.ptr)(n, type_, lists) }
     #[inline] pub unsafe fn Clear(mask: GLbitfield) -> () { mem::transmute::<_, extern "system" fn(GLbitfield) -> ()>(storage::Clear.ptr)(mask) }
     #[inline] pub unsafe fn ClearAccum(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::ClearAccum.ptr)(red, green, blue, alpha) }
     #[inline] pub unsafe fn ClearColor(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::ClearColor.ptr)(red, green, blue, alpha) }
     #[inline] pub unsafe fn ClearDepth(depth: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble) -> ()>(storage::ClearDepth.ptr)(depth) }
     #[inline] pub unsafe fn ClearIndex(c: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(storage::ClearIndex.ptr)(c) }
     #[inline] pub unsafe fn ClearStencil(s: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint) -> ()>(storage::ClearStencil.ptr)(s) }
     #[inline] pub unsafe fn ClipPlane(plane: GLenum, equation: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLdouble) -> ()>(storage::ClipPlane.ptr)(plane, equation) }
     #[inline] pub unsafe fn Color3b(red: GLbyte, green: GLbyte, blue: GLbyte) -> () { mem::transmute::<_, extern "system" fn(GLbyte, GLbyte, GLbyte) -> ()>(storage::Color3b.ptr)(red, green, blue) }
     #[inline] pub unsafe fn Color3bv(v: *const GLbyte) -> () { mem::transmute::<_, extern "system" fn(*const GLbyte) -> ()>(storage::Color3bv.ptr)(v) }
     #[inline] pub unsafe fn Color3d(red: GLdouble, green: GLdouble, blue: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble, GLdouble) -> ()>(storage::Color3d.ptr)(red, green, blue) }
     #[inline] pub unsafe fn Color3dv(v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::Color3dv.ptr)(v) }
     #[inline] pub unsafe fn Color3f(red: GLfloat, green: GLfloat, blue: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat) -> ()>(storage::Color3f.ptr)(red, green, blue) }
     #[inline] pub unsafe fn Color3fv(v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::Color3fv.ptr)(v) }
     #[inline] pub unsafe fn Color3i(red: GLint, green: GLint, blue: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint) -> ()>(storage::Color3i.ptr)(red, green, blue) }
     #[inline] pub unsafe fn Color3iv(v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint) -> ()>(storage::Color3iv.ptr)(v) }
     #[inline] pub unsafe fn Color3s(red: GLshort, green: GLshort, blue: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort, GLshort, GLshort) -> ()>(storage::Color3s.ptr)(red, green, blue) }
     #[inline] pub unsafe fn Color3sv(v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort) -> ()>(storage::Color3sv.ptr)(v) }
     #[inline] pub unsafe fn Color3ub(red: GLubyte, green: GLubyte, blue: GLubyte) -> () { mem::transmute::<_, extern "system" fn(GLubyte, GLubyte, GLubyte) -> ()>(storage::Color3ub.ptr)(red, green, blue) }
     #[inline] pub unsafe fn Color3ubv(v: *const GLubyte) -> () { mem::transmute::<_, extern "system" fn(*const GLubyte) -> ()>(storage::Color3ubv.ptr)(v) }
     #[inline] pub unsafe fn Color3ui(red: GLuint, green: GLuint, blue: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint) -> ()>(storage::Color3ui.ptr)(red, green, blue) }
     #[inline] pub unsafe fn Color3uiv(v: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(*const GLuint) -> ()>(storage::Color3uiv.ptr)(v) }
     #[inline] pub unsafe fn Color3us(red: GLushort, green: GLushort, blue: GLushort) -> () { mem::transmute::<_, extern "system" fn(GLushort, GLushort, GLushort) -> ()>(storage::Color3us.ptr)(red, green, blue) }
     #[inline] pub unsafe fn Color3usv(v: *const GLushort) -> () { mem::transmute::<_, extern "system" fn(*const GLushort) -> ()>(storage::Color3usv.ptr)(v) }
     #[inline] pub unsafe fn Color4b(red: GLbyte, green: GLbyte, blue: GLbyte, alpha: GLbyte) -> () { mem::transmute::<_, extern "system" fn(GLbyte, GLbyte, GLbyte, GLbyte) -> ()>(storage::Color4b.ptr)(red, green, blue, alpha) }
     #[inline] pub unsafe fn Color4bv(v: *const GLbyte) -> () { mem::transmute::<_, extern "system" fn(*const GLbyte) -> ()>(storage::Color4bv.ptr)(v) }
     #[inline] pub unsafe fn Color4d(red: GLdouble, green: GLdouble, blue: GLdouble, alpha: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble, GLdouble, GLdouble) -> ()>(storage::Color4d.ptr)(red, green, blue, alpha) }
     #[inline] pub unsafe fn Color4dv(v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::Color4dv.ptr)(v) }
     #[inline] pub unsafe fn Color4f(red: GLfloat, green: GLfloat, blue: GLfloat, alpha: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::Color4f.ptr)(red, green, blue, alpha) }
     #[inline] pub unsafe fn Color4fv(v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::Color4fv.ptr)(v) }
     #[inline] pub unsafe fn Color4i(red: GLint, green: GLint, blue: GLint, alpha: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint, GLint) -> ()>(storage::Color4i.ptr)(red, green, blue, alpha) }
     #[inline] pub unsafe fn Color4iv(v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint) -> ()>(storage::Color4iv.ptr)(v) }
     #[inline] pub unsafe fn Color4s(red: GLshort, green: GLshort, blue: GLshort, alpha: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort, GLshort, GLshort, GLshort) -> ()>(storage::Color4s.ptr)(red, green, blue, alpha) }
     #[inline] pub unsafe fn Color4sv(v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort) -> ()>(storage::Color4sv.ptr)(v) }
     #[inline] pub unsafe fn Color4ub(red: GLubyte, green: GLubyte, blue: GLubyte, alpha: GLubyte) -> () { mem::transmute::<_, extern "system" fn(GLubyte, GLubyte, GLubyte, GLubyte) -> ()>(storage::Color4ub.ptr)(red, green, blue, alpha) }
     #[inline] pub unsafe fn Color4ubv(v: *const GLubyte) -> () { mem::transmute::<_, extern "system" fn(*const GLubyte) -> ()>(storage::Color4ubv.ptr)(v) }
     #[inline] pub unsafe fn Color4ui(red: GLuint, green: GLuint, blue: GLuint, alpha: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLuint, GLuint, GLuint) -> ()>(storage::Color4ui.ptr)(red, green, blue, alpha) }
     #[inline] pub unsafe fn Color4uiv(v: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(*const GLuint) -> ()>(storage::Color4uiv.ptr)(v) }
     #[inline] pub unsafe fn Color4us(red: GLushort, green: GLushort, blue: GLushort, alpha: GLushort) -> () { mem::transmute::<_, extern "system" fn(GLushort, GLushort, GLushort, GLushort) -> ()>(storage::Color4us.ptr)(red, green, blue, alpha) }
     #[inline] pub unsafe fn Color4usv(v: *const GLushort) -> () { mem::transmute::<_, extern "system" fn(*const GLushort) -> ()>(storage::Color4usv.ptr)(v) }
     #[inline] pub unsafe fn ColorMask(red: GLboolean, green: GLboolean, blue: GLboolean, alpha: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLboolean, GLboolean, GLboolean, GLboolean) -> ()>(storage::ColorMask.ptr)(red, green, blue, alpha) }
     #[inline] pub unsafe fn ColorMaterial(face: GLenum, mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(storage::ColorMaterial.ptr)(face, mode) }
     #[inline] pub unsafe fn CopyPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, type_: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLsizei, GLsizei, GLenum) -> ()>(storage::CopyPixels.ptr)(x, y, width, height, type_) }
     #[inline] pub unsafe fn CullFace(mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::CullFace.ptr)(mode) }
     #[inline] pub unsafe fn DeleteLists(list: GLuint, range: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLsizei) -> ()>(storage::DeleteLists.ptr)(list, range) }
     #[inline] pub unsafe fn DepthFunc(func: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::DepthFunc.ptr)(func) }
     #[inline] pub unsafe fn DepthMask(flag: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLboolean) -> ()>(storage::DepthMask.ptr)(flag) }
     #[inline] pub unsafe fn DepthRange(n: GLdouble, f: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble) -> ()>(storage::DepthRange.ptr)(n, f) }
     #[inline] pub unsafe fn Disable(cap: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::Disable.ptr)(cap) }
     #[inline] pub unsafe fn DrawBuffer(buf: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::DrawBuffer.ptr)(buf) }
     #[inline] pub unsafe fn DrawPixels(width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLsizei, GLsizei, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::DrawPixels.ptr)(width, height, format, type_, pixels) }
     #[inline] pub unsafe fn EdgeFlag(flag: GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLboolean) -> ()>(storage::EdgeFlag.ptr)(flag) }
     #[inline] pub unsafe fn EdgeFlagv(flag: *const GLboolean) -> () { mem::transmute::<_, extern "system" fn(*const GLboolean) -> ()>(storage::EdgeFlagv.ptr)(flag) }
     #[inline] pub unsafe fn Enable(cap: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::Enable.ptr)(cap) }
     #[inline] pub unsafe fn End() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::End.ptr)() }
     #[inline] pub unsafe fn EndList() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::EndList.ptr)() }
     #[inline] pub unsafe fn EvalCoord1d(u: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble) -> ()>(storage::EvalCoord1d.ptr)(u) }
     #[inline] pub unsafe fn EvalCoord1dv(u: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::EvalCoord1dv.ptr)(u) }
     #[inline] pub unsafe fn EvalCoord1f(u: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(storage::EvalCoord1f.ptr)(u) }
     #[inline] pub unsafe fn EvalCoord1fv(u: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::EvalCoord1fv.ptr)(u) }
     #[inline] pub unsafe fn EvalCoord2d(u: GLdouble, v: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble) -> ()>(storage::EvalCoord2d.ptr)(u, v) }
     #[inline] pub unsafe fn EvalCoord2dv(u: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::EvalCoord2dv.ptr)(u) }
     #[inline] pub unsafe fn EvalCoord2f(u: GLfloat, v: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat) -> ()>(storage::EvalCoord2f.ptr)(u, v) }
     #[inline] pub unsafe fn EvalCoord2fv(u: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::EvalCoord2fv.ptr)(u) }
     #[inline] pub unsafe fn EvalMesh1(mode: GLenum, i1: GLint, i2: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint) -> ()>(storage::EvalMesh1.ptr)(mode, i1, i2) }
     #[inline] pub unsafe fn EvalMesh2(mode: GLenum, i1: GLint, i2: GLint, j1: GLint, j2: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLint, GLint) -> ()>(storage::EvalMesh2.ptr)(mode, i1, i2, j1, j2) }
     #[inline] pub unsafe fn EvalPoint1(i: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint) -> ()>(storage::EvalPoint1.ptr)(i) }
     #[inline] pub unsafe fn EvalPoint2(i: GLint, j: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint) -> ()>(storage::EvalPoint2.ptr)(i, j) }
     #[inline] pub unsafe fn FeedbackBuffer(size: GLsizei, type_: GLenum, buffer: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLsizei, GLenum, *mut GLfloat) -> ()>(storage::FeedbackBuffer.ptr)(size, type_, buffer) }
     #[inline] pub unsafe fn Finish() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::Finish.ptr)() }
     #[inline] pub unsafe fn Flush() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::Flush.ptr)() }
     #[inline] pub unsafe fn Fogf(pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLfloat) -> ()>(storage::Fogf.ptr)(pname, param) }
     #[inline] pub unsafe fn Fogfv(pname: GLenum, params: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLfloat) -> ()>(storage::Fogfv.ptr)(pname, params) }
     #[inline] pub unsafe fn Fogi(pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint) -> ()>(storage::Fogi.ptr)(pname, param) }
     #[inline] pub unsafe fn Fogiv(pname: GLenum, params: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLint) -> ()>(storage::Fogiv.ptr)(pname, params) }
     #[inline] pub unsafe fn FrontFace(mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::FrontFace.ptr)(mode) }
     #[inline] pub unsafe fn Frustum(left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble, GLdouble, GLdouble, GLdouble, GLdouble) -> ()>(storage::Frustum.ptr)(left, right, bottom, top, zNear, zFar) }
     #[inline] pub unsafe fn GenLists(range: GLsizei) -> GLuint { mem::transmute::<_, extern "system" fn(GLsizei) -> GLuint>(storage::GenLists.ptr)(range) }
     #[inline] pub unsafe fn GetBooleanv(pname: GLenum, data: *mut GLboolean) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLboolean) -> ()>(storage::GetBooleanv.ptr)(pname, data) }
     #[inline] pub unsafe fn GetClipPlane(plane: GLenum, equation: *mut GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLdouble) -> ()>(storage::GetClipPlane.ptr)(plane, equation) }
     #[inline] pub unsafe fn GetDoublev(pname: GLenum, data: *mut GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLdouble) -> ()>(storage::GetDoublev.ptr)(pname, data) }
     #[inline] pub unsafe fn GetError() -> GLenum { mem::transmute::<_, extern "system" fn() -> GLenum>(storage::GetError.ptr)() }
     #[inline] pub unsafe fn GetFloatv(pname: GLenum, data: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLfloat) -> ()>(storage::GetFloatv.ptr)(pname, data) }
     #[inline] pub unsafe fn GetIntegerv(pname: GLenum, data: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLint) -> ()>(storage::GetIntegerv.ptr)(pname, data) }
     #[inline] pub unsafe fn GetLightfv(light: GLenum, pname: GLenum, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLfloat) -> ()>(storage::GetLightfv.ptr)(light, pname, params) }
     #[inline] pub unsafe fn GetLightiv(light: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetLightiv.ptr)(light, pname, params) }
     #[inline] pub unsafe fn GetMapdv(target: GLenum, query: GLenum, v: *mut GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLdouble) -> ()>(storage::GetMapdv.ptr)(target, query, v) }
     #[inline] pub unsafe fn GetMapfv(target: GLenum, query: GLenum, v: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLfloat) -> ()>(storage::GetMapfv.ptr)(target, query, v) }
     #[inline] pub unsafe fn GetMapiv(target: GLenum, query: GLenum, v: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetMapiv.ptr)(target, query, v) }
     #[inline] pub unsafe fn GetMaterialfv(face: GLenum, pname: GLenum, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLfloat) -> ()>(storage::GetMaterialfv.ptr)(face, pname, params) }
     #[inline] pub unsafe fn GetMaterialiv(face: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetMaterialiv.ptr)(face, pname, params) }
     #[inline] pub unsafe fn GetPixelMapfv(map: GLenum, values: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLfloat) -> ()>(storage::GetPixelMapfv.ptr)(map, values) }
     #[inline] pub unsafe fn GetPixelMapuiv(map: GLenum, values: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLuint) -> ()>(storage::GetPixelMapuiv.ptr)(map, values) }
     #[inline] pub unsafe fn GetPixelMapusv(map: GLenum, values: *mut GLushort) -> () { mem::transmute::<_, extern "system" fn(GLenum, *mut GLushort) -> ()>(storage::GetPixelMapusv.ptr)(map, values) }
     #[inline] pub unsafe fn GetPolygonStipple(mask: *mut GLubyte) -> () { mem::transmute::<_, extern "system" fn(*mut GLubyte) -> ()>(storage::GetPolygonStipple.ptr)(mask) }
     #[inline] pub unsafe fn GetString(name: GLenum) -> *const GLubyte { mem::transmute::<_, extern "system" fn(GLenum) -> *const GLubyte>(storage::GetString.ptr)(name) }
     #[inline] pub unsafe fn GetTexEnvfv(target: GLenum, pname: GLenum, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLfloat) -> ()>(storage::GetTexEnvfv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetTexEnviv(target: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetTexEnviv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetTexGendv(coord: GLenum, pname: GLenum, params: *mut GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLdouble) -> ()>(storage::GetTexGendv.ptr)(coord, pname, params) }
     #[inline] pub unsafe fn GetTexGenfv(coord: GLenum, pname: GLenum, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLfloat) -> ()>(storage::GetTexGenfv.ptr)(coord, pname, params) }
     #[inline] pub unsafe fn GetTexGeniv(coord: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetTexGeniv.ptr)(coord, pname, params) }
     #[inline] pub unsafe fn GetTexImage(target: GLenum, level: GLint, format: GLenum, type_: GLenum, pixels: *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, GLenum, *mut std::os::raw::c_void) -> ()>(storage::GetTexImage.ptr)(target, level, format, type_, pixels) }
     #[inline] pub unsafe fn GetTexLevelParameterfv(target: GLenum, level: GLint, pname: GLenum, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, *mut GLfloat) -> ()>(storage::GetTexLevelParameterfv.ptr)(target, level, pname, params) }
     #[inline] pub unsafe fn GetTexLevelParameteriv(target: GLenum, level: GLint, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLenum, *mut GLint) -> ()>(storage::GetTexLevelParameteriv.ptr)(target, level, pname, params) }
     #[inline] pub unsafe fn GetTexParameterfv(target: GLenum, pname: GLenum, params: *mut GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLfloat) -> ()>(storage::GetTexParameterfv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn GetTexParameteriv(target: GLenum, pname: GLenum, params: *mut GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *mut GLint) -> ()>(storage::GetTexParameteriv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn Hint(target: GLenum, mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(storage::Hint.ptr)(target, mode) }
     #[inline] pub unsafe fn IndexMask(mask: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::IndexMask.ptr)(mask) }
     #[inline] pub unsafe fn Indexd(c: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble) -> ()>(storage::Indexd.ptr)(c) }
     #[inline] pub unsafe fn Indexdv(c: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::Indexdv.ptr)(c) }
     #[inline] pub unsafe fn Indexf(c: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(storage::Indexf.ptr)(c) }
     #[inline] pub unsafe fn Indexfv(c: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::Indexfv.ptr)(c) }
     #[inline] pub unsafe fn Indexi(c: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint) -> ()>(storage::Indexi.ptr)(c) }
     #[inline] pub unsafe fn Indexiv(c: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint) -> ()>(storage::Indexiv.ptr)(c) }
     #[inline] pub unsafe fn Indexs(c: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort) -> ()>(storage::Indexs.ptr)(c) }
     #[inline] pub unsafe fn Indexsv(c: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort) -> ()>(storage::Indexsv.ptr)(c) }
     #[inline] pub unsafe fn InitNames() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::InitNames.ptr)() }
     #[inline] pub unsafe fn IsEnabled(cap: GLenum) -> GLboolean { mem::transmute::<_, extern "system" fn(GLenum) -> GLboolean>(storage::IsEnabled.ptr)(cap) }
     #[inline] pub unsafe fn IsList(list: GLuint) -> GLboolean { mem::transmute::<_, extern "system" fn(GLuint) -> GLboolean>(storage::IsList.ptr)(list) }
     #[inline] pub unsafe fn LightModelf(pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLfloat) -> ()>(storage::LightModelf.ptr)(pname, param) }
     #[inline] pub unsafe fn LightModelfv(pname: GLenum, params: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLfloat) -> ()>(storage::LightModelfv.ptr)(pname, params) }
     #[inline] pub unsafe fn LightModeli(pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint) -> ()>(storage::LightModeli.ptr)(pname, param) }
     #[inline] pub unsafe fn LightModeliv(pname: GLenum, params: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, *const GLint) -> ()>(storage::LightModeliv.ptr)(pname, params) }
     #[inline] pub unsafe fn Lightf(light: GLenum, pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLfloat) -> ()>(storage::Lightf.ptr)(light, pname, param) }
     #[inline] pub unsafe fn Lightfv(light: GLenum, pname: GLenum, params: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLfloat) -> ()>(storage::Lightfv.ptr)(light, pname, params) }
     #[inline] pub unsafe fn Lighti(light: GLenum, pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint) -> ()>(storage::Lighti.ptr)(light, pname, param) }
     #[inline] pub unsafe fn Lightiv(light: GLenum, pname: GLenum, params: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLint) -> ()>(storage::Lightiv.ptr)(light, pname, params) }
     #[inline] pub unsafe fn LineStipple(factor: GLint, pattern: GLushort) -> () { mem::transmute::<_, extern "system" fn(GLint, GLushort) -> ()>(storage::LineStipple.ptr)(factor, pattern) }
     #[inline] pub unsafe fn LineWidth(width: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(storage::LineWidth.ptr)(width) }
     #[inline] pub unsafe fn ListBase(base: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::ListBase.ptr)(base) }
     #[inline] pub unsafe fn LoadIdentity() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::LoadIdentity.ptr)() }
     #[inline] pub unsafe fn LoadMatrixd(m: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::LoadMatrixd.ptr)(m) }
     #[inline] pub unsafe fn LoadMatrixf(m: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::LoadMatrixf.ptr)(m) }
     #[inline] pub unsafe fn LoadName(name: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::LoadName.ptr)(name) }
     #[inline] pub unsafe fn LogicOp(opcode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::LogicOp.ptr)(opcode) }
     #[inline] pub unsafe fn Map1d(target: GLenum, u1: GLdouble, u2: GLdouble, stride: GLint, order: GLint, points: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLdouble, GLdouble, GLint, GLint, *const GLdouble) -> ()>(storage::Map1d.ptr)(target, u1, u2, stride, order, points) }
     #[inline] pub unsafe fn Map1f(target: GLenum, u1: GLfloat, u2: GLfloat, stride: GLint, order: GLint, points: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLfloat, GLfloat, GLint, GLint, *const GLfloat) -> ()>(storage::Map1f.ptr)(target, u1, u2, stride, order, points) }
     #[inline] pub unsafe fn Map2d(target: GLenum, u1: GLdouble, u2: GLdouble, ustride: GLint, uorder: GLint, v1: GLdouble, v2: GLdouble, vstride: GLint, vorder: GLint, points: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLdouble, GLdouble, GLint, GLint, GLdouble, GLdouble, GLint, GLint, *const GLdouble) -> ()>(storage::Map2d.ptr)(target, u1, u2, ustride, uorder, v1, v2, vstride, vorder, points) }
     #[inline] pub unsafe fn Map2f(target: GLenum, u1: GLfloat, u2: GLfloat, ustride: GLint, uorder: GLint, v1: GLfloat, v2: GLfloat, vstride: GLint, vorder: GLint, points: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLfloat, GLfloat, GLint, GLint, GLfloat, GLfloat, GLint, GLint, *const GLfloat) -> ()>(storage::Map2f.ptr)(target, u1, u2, ustride, uorder, v1, v2, vstride, vorder, points) }
     #[inline] pub unsafe fn MapGrid1d(un: GLint, u1: GLdouble, u2: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLdouble, GLdouble) -> ()>(storage::MapGrid1d.ptr)(un, u1, u2) }
     #[inline] pub unsafe fn MapGrid1f(un: GLint, u1: GLfloat, u2: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLfloat, GLfloat) -> ()>(storage::MapGrid1f.ptr)(un, u1, u2) }
     #[inline] pub unsafe fn MapGrid2d(un: GLint, u1: GLdouble, u2: GLdouble, vn: GLint, v1: GLdouble, v2: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLint, GLdouble, GLdouble, GLint, GLdouble, GLdouble) -> ()>(storage::MapGrid2d.ptr)(un, u1, u2, vn, v1, v2) }
     #[inline] pub unsafe fn MapGrid2f(un: GLint, u1: GLfloat, u2: GLfloat, vn: GLint, v1: GLfloat, v2: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLint, GLfloat, GLfloat, GLint, GLfloat, GLfloat) -> ()>(storage::MapGrid2f.ptr)(un, u1, u2, vn, v1, v2) }
     #[inline] pub unsafe fn Materialf(face: GLenum, pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLfloat) -> ()>(storage::Materialf.ptr)(face, pname, param) }
     #[inline] pub unsafe fn Materialfv(face: GLenum, pname: GLenum, params: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLfloat) -> ()>(storage::Materialfv.ptr)(face, pname, params) }
     #[inline] pub unsafe fn Materiali(face: GLenum, pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint) -> ()>(storage::Materiali.ptr)(face, pname, param) }
     #[inline] pub unsafe fn Materialiv(face: GLenum, pname: GLenum, params: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLint) -> ()>(storage::Materialiv.ptr)(face, pname, params) }
     #[inline] pub unsafe fn MatrixMode(mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::MatrixMode.ptr)(mode) }
     #[inline] pub unsafe fn MultMatrixd(m: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::MultMatrixd.ptr)(m) }
     #[inline] pub unsafe fn MultMatrixf(m: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::MultMatrixf.ptr)(m) }
     #[inline] pub unsafe fn NewList(list: GLuint, mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLuint, GLenum) -> ()>(storage::NewList.ptr)(list, mode) }
     #[inline] pub unsafe fn Normal3b(nx: GLbyte, ny: GLbyte, nz: GLbyte) -> () { mem::transmute::<_, extern "system" fn(GLbyte, GLbyte, GLbyte) -> ()>(storage::Normal3b.ptr)(nx, ny, nz) }
     #[inline] pub unsafe fn Normal3bv(v: *const GLbyte) -> () { mem::transmute::<_, extern "system" fn(*const GLbyte) -> ()>(storage::Normal3bv.ptr)(v) }
     #[inline] pub unsafe fn Normal3d(nx: GLdouble, ny: GLdouble, nz: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble, GLdouble) -> ()>(storage::Normal3d.ptr)(nx, ny, nz) }
     #[inline] pub unsafe fn Normal3dv(v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::Normal3dv.ptr)(v) }
     #[inline] pub unsafe fn Normal3f(nx: GLfloat, ny: GLfloat, nz: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat) -> ()>(storage::Normal3f.ptr)(nx, ny, nz) }
     #[inline] pub unsafe fn Normal3fv(v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::Normal3fv.ptr)(v) }
     #[inline] pub unsafe fn Normal3i(nx: GLint, ny: GLint, nz: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint) -> ()>(storage::Normal3i.ptr)(nx, ny, nz) }
     #[inline] pub unsafe fn Normal3iv(v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint) -> ()>(storage::Normal3iv.ptr)(v) }
     #[inline] pub unsafe fn Normal3s(nx: GLshort, ny: GLshort, nz: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort, GLshort, GLshort) -> ()>(storage::Normal3s.ptr)(nx, ny, nz) }
     #[inline] pub unsafe fn Normal3sv(v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort) -> ()>(storage::Normal3sv.ptr)(v) }
     #[inline] pub unsafe fn Ortho(left: GLdouble, right: GLdouble, bottom: GLdouble, top: GLdouble, zNear: GLdouble, zFar: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble, GLdouble, GLdouble, GLdouble, GLdouble) -> ()>(storage::Ortho.ptr)(left, right, bottom, top, zNear, zFar) }
     #[inline] pub unsafe fn PassThrough(token: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(storage::PassThrough.ptr)(token) }
     #[inline] pub unsafe fn PixelMapfv(map: GLenum, mapsize: GLsizei, values: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *const GLfloat) -> ()>(storage::PixelMapfv.ptr)(map, mapsize, values) }
     #[inline] pub unsafe fn PixelMapuiv(map: GLenum, mapsize: GLsizei, values: *const GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *const GLuint) -> ()>(storage::PixelMapuiv.ptr)(map, mapsize, values) }
     #[inline] pub unsafe fn PixelMapusv(map: GLenum, mapsize: GLsizei, values: *const GLushort) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLsizei, *const GLushort) -> ()>(storage::PixelMapusv.ptr)(map, mapsize, values) }
     #[inline] pub unsafe fn PixelStoref(pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLfloat) -> ()>(storage::PixelStoref.ptr)(pname, param) }
     #[inline] pub unsafe fn PixelStorei(pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint) -> ()>(storage::PixelStorei.ptr)(pname, param) }
     #[inline] pub unsafe fn PixelTransferf(pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLfloat) -> ()>(storage::PixelTransferf.ptr)(pname, param) }
     #[inline] pub unsafe fn PixelTransferi(pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint) -> ()>(storage::PixelTransferi.ptr)(pname, param) }
     #[inline] pub unsafe fn PixelZoom(xfactor: GLfloat, yfactor: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat) -> ()>(storage::PixelZoom.ptr)(xfactor, yfactor) }
     #[inline] pub unsafe fn PointSize(size: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(storage::PointSize.ptr)(size) }
     #[inline] pub unsafe fn PolygonMode(face: GLenum, mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum) -> ()>(storage::PolygonMode.ptr)(face, mode) }
     #[inline] pub unsafe fn PolygonStipple(mask: *const GLubyte) -> () { mem::transmute::<_, extern "system" fn(*const GLubyte) -> ()>(storage::PolygonStipple.ptr)(mask) }
     #[inline] pub unsafe fn PopAttrib() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::PopAttrib.ptr)() }
     #[inline] pub unsafe fn PopMatrix() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::PopMatrix.ptr)() }
     #[inline] pub unsafe fn PopName() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::PopName.ptr)() }
     #[inline] pub unsafe fn PushAttrib(mask: GLbitfield) -> () { mem::transmute::<_, extern "system" fn(GLbitfield) -> ()>(storage::PushAttrib.ptr)(mask) }
     #[inline] pub unsafe fn PushMatrix() -> () { mem::transmute::<_, extern "system" fn() -> ()>(storage::PushMatrix.ptr)() }
     #[inline] pub unsafe fn PushName(name: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::PushName.ptr)(name) }
     #[inline] pub unsafe fn RasterPos2d(x: GLdouble, y: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble) -> ()>(storage::RasterPos2d.ptr)(x, y) }
     #[inline] pub unsafe fn RasterPos2dv(v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::RasterPos2dv.ptr)(v) }
     #[inline] pub unsafe fn RasterPos2f(x: GLfloat, y: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat) -> ()>(storage::RasterPos2f.ptr)(x, y) }
     #[inline] pub unsafe fn RasterPos2fv(v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::RasterPos2fv.ptr)(v) }
     #[inline] pub unsafe fn RasterPos2i(x: GLint, y: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint) -> ()>(storage::RasterPos2i.ptr)(x, y) }
     #[inline] pub unsafe fn RasterPos2iv(v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint) -> ()>(storage::RasterPos2iv.ptr)(v) }
     #[inline] pub unsafe fn RasterPos2s(x: GLshort, y: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort, GLshort) -> ()>(storage::RasterPos2s.ptr)(x, y) }
     #[inline] pub unsafe fn RasterPos2sv(v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort) -> ()>(storage::RasterPos2sv.ptr)(v) }
     #[inline] pub unsafe fn RasterPos3d(x: GLdouble, y: GLdouble, z: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble, GLdouble) -> ()>(storage::RasterPos3d.ptr)(x, y, z) }
     #[inline] pub unsafe fn RasterPos3dv(v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::RasterPos3dv.ptr)(v) }
     #[inline] pub unsafe fn RasterPos3f(x: GLfloat, y: GLfloat, z: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat) -> ()>(storage::RasterPos3f.ptr)(x, y, z) }
     #[inline] pub unsafe fn RasterPos3fv(v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::RasterPos3fv.ptr)(v) }
     #[inline] pub unsafe fn RasterPos3i(x: GLint, y: GLint, z: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint) -> ()>(storage::RasterPos3i.ptr)(x, y, z) }
     #[inline] pub unsafe fn RasterPos3iv(v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint) -> ()>(storage::RasterPos3iv.ptr)(v) }
     #[inline] pub unsafe fn RasterPos3s(x: GLshort, y: GLshort, z: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort, GLshort, GLshort) -> ()>(storage::RasterPos3s.ptr)(x, y, z) }
     #[inline] pub unsafe fn RasterPos3sv(v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort) -> ()>(storage::RasterPos3sv.ptr)(v) }
     #[inline] pub unsafe fn RasterPos4d(x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble, GLdouble, GLdouble) -> ()>(storage::RasterPos4d.ptr)(x, y, z, w) }
     #[inline] pub unsafe fn RasterPos4dv(v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::RasterPos4dv.ptr)(v) }
     #[inline] pub unsafe fn RasterPos4f(x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::RasterPos4f.ptr)(x, y, z, w) }
     #[inline] pub unsafe fn RasterPos4fv(v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::RasterPos4fv.ptr)(v) }
     #[inline] pub unsafe fn RasterPos4i(x: GLint, y: GLint, z: GLint, w: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint, GLint) -> ()>(storage::RasterPos4i.ptr)(x, y, z, w) }
     #[inline] pub unsafe fn RasterPos4iv(v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint) -> ()>(storage::RasterPos4iv.ptr)(v) }
     #[inline] pub unsafe fn RasterPos4s(x: GLshort, y: GLshort, z: GLshort, w: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort, GLshort, GLshort, GLshort) -> ()>(storage::RasterPos4s.ptr)(x, y, z, w) }
     #[inline] pub unsafe fn RasterPos4sv(v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort) -> ()>(storage::RasterPos4sv.ptr)(v) }
     #[inline] pub unsafe fn ReadBuffer(src: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::ReadBuffer.ptr)(src) }
     #[inline] pub unsafe fn ReadPixels(x: GLint, y: GLint, width: GLsizei, height: GLsizei, format: GLenum, type_: GLenum, pixels: *mut std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLsizei, GLsizei, GLenum, GLenum, *mut std::os::raw::c_void) -> ()>(storage::ReadPixels.ptr)(x, y, width, height, format, type_, pixels) }
     #[inline] pub unsafe fn Rectd(x1: GLdouble, y1: GLdouble, x2: GLdouble, y2: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble, GLdouble, GLdouble) -> ()>(storage::Rectd.ptr)(x1, y1, x2, y2) }
     #[inline] pub unsafe fn Rectdv(v1: *const GLdouble, v2: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble, *const GLdouble) -> ()>(storage::Rectdv.ptr)(v1, v2) }
     #[inline] pub unsafe fn Rectf(x1: GLfloat, y1: GLfloat, x2: GLfloat, y2: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::Rectf.ptr)(x1, y1, x2, y2) }
     #[inline] pub unsafe fn Rectfv(v1: *const GLfloat, v2: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat, *const GLfloat) -> ()>(storage::Rectfv.ptr)(v1, v2) }
     #[inline] pub unsafe fn Recti(x1: GLint, y1: GLint, x2: GLint, y2: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint, GLint) -> ()>(storage::Recti.ptr)(x1, y1, x2, y2) }
     #[inline] pub unsafe fn Rectiv(v1: *const GLint, v2: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint, *const GLint) -> ()>(storage::Rectiv.ptr)(v1, v2) }
     #[inline] pub unsafe fn Rects(x1: GLshort, y1: GLshort, x2: GLshort, y2: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort, GLshort, GLshort, GLshort) -> ()>(storage::Rects.ptr)(x1, y1, x2, y2) }
     #[inline] pub unsafe fn Rectsv(v1: *const GLshort, v2: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort, *const GLshort) -> ()>(storage::Rectsv.ptr)(v1, v2) }
     #[inline] pub unsafe fn RenderMode(mode: GLenum) -> GLint { mem::transmute::<_, extern "system" fn(GLenum) -> GLint>(storage::RenderMode.ptr)(mode) }
     #[inline] pub unsafe fn Rotated(angle: GLdouble, x: GLdouble, y: GLdouble, z: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble, GLdouble, GLdouble) -> ()>(storage::Rotated.ptr)(angle, x, y, z) }
     #[inline] pub unsafe fn Rotatef(angle: GLfloat, x: GLfloat, y: GLfloat, z: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::Rotatef.ptr)(angle, x, y, z) }
     #[inline] pub unsafe fn Scaled(x: GLdouble, y: GLdouble, z: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble, GLdouble) -> ()>(storage::Scaled.ptr)(x, y, z) }
     #[inline] pub unsafe fn Scalef(x: GLfloat, y: GLfloat, z: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat) -> ()>(storage::Scalef.ptr)(x, y, z) }
     #[inline] pub unsafe fn Scissor(x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLsizei, GLsizei) -> ()>(storage::Scissor.ptr)(x, y, width, height) }
     #[inline] pub unsafe fn SelectBuffer(size: GLsizei, buffer: *mut GLuint) -> () { mem::transmute::<_, extern "system" fn(GLsizei, *mut GLuint) -> ()>(storage::SelectBuffer.ptr)(size, buffer) }
     #[inline] pub unsafe fn ShadeModel(mode: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum) -> ()>(storage::ShadeModel.ptr)(mode) }
     #[inline] pub unsafe fn StencilFunc(func: GLenum, ref_: GLint, mask: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLuint) -> ()>(storage::StencilFunc.ptr)(func, ref_, mask) }
     #[inline] pub unsafe fn StencilMask(mask: GLuint) -> () { mem::transmute::<_, extern "system" fn(GLuint) -> ()>(storage::StencilMask.ptr)(mask) }
     #[inline] pub unsafe fn StencilOp(fail: GLenum, zfail: GLenum, zpass: GLenum) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLenum) -> ()>(storage::StencilOp.ptr)(fail, zfail, zpass) }
     #[inline] pub unsafe fn TexCoord1d(s: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble) -> ()>(storage::TexCoord1d.ptr)(s) }
     #[inline] pub unsafe fn TexCoord1dv(v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::TexCoord1dv.ptr)(v) }
     #[inline] pub unsafe fn TexCoord1f(s: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat) -> ()>(storage::TexCoord1f.ptr)(s) }
     #[inline] pub unsafe fn TexCoord1fv(v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::TexCoord1fv.ptr)(v) }
     #[inline] pub unsafe fn TexCoord1i(s: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint) -> ()>(storage::TexCoord1i.ptr)(s) }
     #[inline] pub unsafe fn TexCoord1iv(v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint) -> ()>(storage::TexCoord1iv.ptr)(v) }
     #[inline] pub unsafe fn TexCoord1s(s: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort) -> ()>(storage::TexCoord1s.ptr)(s) }
     #[inline] pub unsafe fn TexCoord1sv(v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort) -> ()>(storage::TexCoord1sv.ptr)(v) }
     #[inline] pub unsafe fn TexCoord2d(s: GLdouble, t: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble) -> ()>(storage::TexCoord2d.ptr)(s, t) }
     #[inline] pub unsafe fn TexCoord2dv(v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::TexCoord2dv.ptr)(v) }
     #[inline] pub unsafe fn TexCoord2f(s: GLfloat, t: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat) -> ()>(storage::TexCoord2f.ptr)(s, t) }
     #[inline] pub unsafe fn TexCoord2fv(v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::TexCoord2fv.ptr)(v) }
     #[inline] pub unsafe fn TexCoord2i(s: GLint, t: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint) -> ()>(storage::TexCoord2i.ptr)(s, t) }
     #[inline] pub unsafe fn TexCoord2iv(v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint) -> ()>(storage::TexCoord2iv.ptr)(v) }
     #[inline] pub unsafe fn TexCoord2s(s: GLshort, t: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort, GLshort) -> ()>(storage::TexCoord2s.ptr)(s, t) }
     #[inline] pub unsafe fn TexCoord2sv(v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort) -> ()>(storage::TexCoord2sv.ptr)(v) }
     #[inline] pub unsafe fn TexCoord3d(s: GLdouble, t: GLdouble, r: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble, GLdouble) -> ()>(storage::TexCoord3d.ptr)(s, t, r) }
     #[inline] pub unsafe fn TexCoord3dv(v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::TexCoord3dv.ptr)(v) }
     #[inline] pub unsafe fn TexCoord3f(s: GLfloat, t: GLfloat, r: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat) -> ()>(storage::TexCoord3f.ptr)(s, t, r) }
     #[inline] pub unsafe fn TexCoord3fv(v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::TexCoord3fv.ptr)(v) }
     #[inline] pub unsafe fn TexCoord3i(s: GLint, t: GLint, r: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint) -> ()>(storage::TexCoord3i.ptr)(s, t, r) }
     #[inline] pub unsafe fn TexCoord3iv(v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint) -> ()>(storage::TexCoord3iv.ptr)(v) }
     #[inline] pub unsafe fn TexCoord3s(s: GLshort, t: GLshort, r: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort, GLshort, GLshort) -> ()>(storage::TexCoord3s.ptr)(s, t, r) }
     #[inline] pub unsafe fn TexCoord3sv(v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort) -> ()>(storage::TexCoord3sv.ptr)(v) }
     #[inline] pub unsafe fn TexCoord4d(s: GLdouble, t: GLdouble, r: GLdouble, q: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble, GLdouble, GLdouble) -> ()>(storage::TexCoord4d.ptr)(s, t, r, q) }
     #[inline] pub unsafe fn TexCoord4dv(v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::TexCoord4dv.ptr)(v) }
     #[inline] pub unsafe fn TexCoord4f(s: GLfloat, t: GLfloat, r: GLfloat, q: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::TexCoord4f.ptr)(s, t, r, q) }
     #[inline] pub unsafe fn TexCoord4fv(v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::TexCoord4fv.ptr)(v) }
     #[inline] pub unsafe fn TexCoord4i(s: GLint, t: GLint, r: GLint, q: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint, GLint) -> ()>(storage::TexCoord4i.ptr)(s, t, r, q) }
     #[inline] pub unsafe fn TexCoord4iv(v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint) -> ()>(storage::TexCoord4iv.ptr)(v) }
     #[inline] pub unsafe fn TexCoord4s(s: GLshort, t: GLshort, r: GLshort, q: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort, GLshort, GLshort, GLshort) -> ()>(storage::TexCoord4s.ptr)(s, t, r, q) }
     #[inline] pub unsafe fn TexCoord4sv(v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort) -> ()>(storage::TexCoord4sv.ptr)(v) }
     #[inline] pub unsafe fn TexEnvf(target: GLenum, pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLfloat) -> ()>(storage::TexEnvf.ptr)(target, pname, param) }
     #[inline] pub unsafe fn TexEnvfv(target: GLenum, pname: GLenum, params: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLfloat) -> ()>(storage::TexEnvfv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn TexEnvi(target: GLenum, pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint) -> ()>(storage::TexEnvi.ptr)(target, pname, param) }
     #[inline] pub unsafe fn TexEnviv(target: GLenum, pname: GLenum, params: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLint) -> ()>(storage::TexEnviv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn TexGend(coord: GLenum, pname: GLenum, param: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLdouble) -> ()>(storage::TexGend.ptr)(coord, pname, param) }
     #[inline] pub unsafe fn TexGendv(coord: GLenum, pname: GLenum, params: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLdouble) -> ()>(storage::TexGendv.ptr)(coord, pname, params) }
     #[inline] pub unsafe fn TexGenf(coord: GLenum, pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLfloat) -> ()>(storage::TexGenf.ptr)(coord, pname, param) }
     #[inline] pub unsafe fn TexGenfv(coord: GLenum, pname: GLenum, params: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLfloat) -> ()>(storage::TexGenfv.ptr)(coord, pname, params) }
     #[inline] pub unsafe fn TexGeni(coord: GLenum, pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint) -> ()>(storage::TexGeni.ptr)(coord, pname, param) }
     #[inline] pub unsafe fn TexGeniv(coord: GLenum, pname: GLenum, params: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLint) -> ()>(storage::TexGeniv.ptr)(coord, pname, params) }
     #[inline] pub unsafe fn TexImage1D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLsizei, GLint, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::TexImage1D.ptr)(target, level, internalformat, width, border, format, type_, pixels) }
     #[inline] pub unsafe fn TexImage2D(target: GLenum, level: GLint, internalformat: GLint, width: GLsizei, height: GLsizei, border: GLint, format: GLenum, type_: GLenum, pixels: *const std::os::raw::c_void) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLint, GLint, GLsizei, GLsizei, GLint, GLenum, GLenum, *const std::os::raw::c_void) -> ()>(storage::TexImage2D.ptr)(target, level, internalformat, width, height, border, format, type_, pixels) }
     #[inline] pub unsafe fn TexParameterf(target: GLenum, pname: GLenum, param: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLfloat) -> ()>(storage::TexParameterf.ptr)(target, pname, param) }
     #[inline] pub unsafe fn TexParameterfv(target: GLenum, pname: GLenum, params: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLfloat) -> ()>(storage::TexParameterfv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn TexParameteri(target: GLenum, pname: GLenum, param: GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, GLint) -> ()>(storage::TexParameteri.ptr)(target, pname, param) }
     #[inline] pub unsafe fn TexParameteriv(target: GLenum, pname: GLenum, params: *const GLint) -> () { mem::transmute::<_, extern "system" fn(GLenum, GLenum, *const GLint) -> ()>(storage::TexParameteriv.ptr)(target, pname, params) }
     #[inline] pub unsafe fn Translated(x: GLdouble, y: GLdouble, z: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble, GLdouble) -> ()>(storage::Translated.ptr)(x, y, z) }
     #[inline] pub unsafe fn Translatef(x: GLfloat, y: GLfloat, z: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat) -> ()>(storage::Translatef.ptr)(x, y, z) }
     #[inline] pub unsafe fn Vertex2d(x: GLdouble, y: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble) -> ()>(storage::Vertex2d.ptr)(x, y) }
     #[inline] pub unsafe fn Vertex2dv(v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::Vertex2dv.ptr)(v) }
     #[inline] pub unsafe fn Vertex2f(x: GLfloat, y: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat) -> ()>(storage::Vertex2f.ptr)(x, y) }
     #[inline] pub unsafe fn Vertex2fv(v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::Vertex2fv.ptr)(v) }
     #[inline] pub unsafe fn Vertex2i(x: GLint, y: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint) -> ()>(storage::Vertex2i.ptr)(x, y) }
     #[inline] pub unsafe fn Vertex2iv(v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint) -> ()>(storage::Vertex2iv.ptr)(v) }
     #[inline] pub unsafe fn Vertex2s(x: GLshort, y: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort, GLshort) -> ()>(storage::Vertex2s.ptr)(x, y) }
     #[inline] pub unsafe fn Vertex2sv(v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort) -> ()>(storage::Vertex2sv.ptr)(v) }
     #[inline] pub unsafe fn Vertex3d(x: GLdouble, y: GLdouble, z: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble, GLdouble) -> ()>(storage::Vertex3d.ptr)(x, y, z) }
     #[inline] pub unsafe fn Vertex3dv(v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::Vertex3dv.ptr)(v) }
     #[inline] pub unsafe fn Vertex3f(x: GLfloat, y: GLfloat, z: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat) -> ()>(storage::Vertex3f.ptr)(x, y, z) }
     #[inline] pub unsafe fn Vertex3fv(v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::Vertex3fv.ptr)(v) }
     #[inline] pub unsafe fn Vertex3i(x: GLint, y: GLint, z: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint) -> ()>(storage::Vertex3i.ptr)(x, y, z) }
     #[inline] pub unsafe fn Vertex3iv(v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint) -> ()>(storage::Vertex3iv.ptr)(v) }
     #[inline] pub unsafe fn Vertex3s(x: GLshort, y: GLshort, z: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort, GLshort, GLshort) -> ()>(storage::Vertex3s.ptr)(x, y, z) }
     #[inline] pub unsafe fn Vertex3sv(v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort) -> ()>(storage::Vertex3sv.ptr)(v) }
     #[inline] pub unsafe fn Vertex4d(x: GLdouble, y: GLdouble, z: GLdouble, w: GLdouble) -> () { mem::transmute::<_, extern "system" fn(GLdouble, GLdouble, GLdouble, GLdouble) -> ()>(storage::Vertex4d.ptr)(x, y, z, w) }
     #[inline] pub unsafe fn Vertex4dv(v: *const GLdouble) -> () { mem::transmute::<_, extern "system" fn(*const GLdouble) -> ()>(storage::Vertex4dv.ptr)(v) }
     #[inline] pub unsafe fn Vertex4f(x: GLfloat, y: GLfloat, z: GLfloat, w: GLfloat) -> () { mem::transmute::<_, extern "system" fn(GLfloat, GLfloat, GLfloat, GLfloat) -> ()>(storage::Vertex4f.ptr)(x, y, z, w) }
     #[inline] pub unsafe fn Vertex4fv(v: *const GLfloat) -> () { mem::transmute::<_, extern "system" fn(*const GLfloat) -> ()>(storage::Vertex4fv.ptr)(v) }
     #[inline] pub unsafe fn Vertex4i(x: GLint, y: GLint, z: GLint, w: GLint) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLint, GLint) -> ()>(storage::Vertex4i.ptr)(x, y, z, w) }
     #[inline] pub unsafe fn Vertex4iv(v: *const GLint) -> () { mem::transmute::<_, extern "system" fn(*const GLint) -> ()>(storage::Vertex4iv.ptr)(v) }
     #[inline] pub unsafe fn Vertex4s(x: GLshort, y: GLshort, z: GLshort, w: GLshort) -> () { mem::transmute::<_, extern "system" fn(GLshort, GLshort, GLshort, GLshort) -> ()>(storage::Vertex4s.ptr)(x, y, z, w) }
     #[inline] pub unsafe fn Vertex4sv(v: *const GLshort) -> () { mem::transmute::<_, extern "system" fn(*const GLshort) -> ()>(storage::Vertex4sv.ptr)(v) }
     #[inline] pub unsafe fn Viewport(x: GLint, y: GLint, width: GLsizei, height: GLsizei) -> () { mem::transmute::<_, extern "system" fn(GLint, GLint, GLsizei, GLsizei) -> ()>(storage::Viewport.ptr)(x, y, width, height) }
}

mod storage {
    #![allow(non_snake_case, non_upper_case_globals)]

    use super::FnPtr;
    use std::os::raw;

     pub static mut Accum: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut AlphaFunc: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Begin: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Bitmap: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut BlendFunc: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CallList: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CallLists: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Clear: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearAccum: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearColor: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearDepth: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearIndex: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClearStencil: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ClipPlane: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color3b: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color3bv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color3d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color3dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color3f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color3fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color3i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color3iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color3s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color3sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color3ub: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color3ubv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color3ui: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color3uiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color3us: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color3usv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color4b: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color4bv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color4d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color4dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color4f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color4fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color4i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color4iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color4s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color4sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color4ub: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color4ubv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color4ui: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color4uiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color4us: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Color4usv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ColorMask: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ColorMaterial: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CopyPixels: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut CullFace: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DeleteLists: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DepthFunc: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DepthMask: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DepthRange: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Disable: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut DrawPixels: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EdgeFlag: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EdgeFlagv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Enable: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut End: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EndList: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EvalCoord1d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EvalCoord1dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EvalCoord1f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EvalCoord1fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EvalCoord2d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EvalCoord2dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EvalCoord2f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EvalCoord2fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EvalMesh1: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EvalMesh2: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EvalPoint1: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut EvalPoint2: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut FeedbackBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Finish: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Flush: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Fogf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Fogfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Fogi: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Fogiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut FrontFace: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Frustum: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GenLists: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetBooleanv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetClipPlane: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetDoublev: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetError: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetFloatv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetIntegerv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetLightfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetLightiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetMapdv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetMapfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetMapiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetMaterialfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetMaterialiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetPixelMapfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetPixelMapuiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetPixelMapusv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetPolygonStipple: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetString: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexEnvfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexEnviv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexGendv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexGenfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexGeniv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexImage: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexLevelParameterfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexLevelParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexParameterfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut GetTexParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Hint: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IndexMask: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Indexd: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Indexdv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Indexf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Indexfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Indexi: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Indexiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Indexs: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Indexsv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut InitNames: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsEnabled: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut IsList: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LightModelf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LightModelfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LightModeli: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LightModeliv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Lightf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Lightfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Lighti: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Lightiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LineStipple: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LineWidth: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ListBase: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LoadIdentity: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LoadMatrixd: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LoadMatrixf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LoadName: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut LogicOp: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Map1d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Map1f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Map2d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Map2f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MapGrid1d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MapGrid1f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MapGrid2d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MapGrid2f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Materialf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Materialfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Materiali: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Materialiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MatrixMode: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultMatrixd: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut MultMatrixf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut NewList: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Normal3b: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Normal3bv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Normal3d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Normal3dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Normal3f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Normal3fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Normal3i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Normal3iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Normal3s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Normal3sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Ortho: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PassThrough: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PixelMapfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PixelMapuiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PixelMapusv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PixelStoref: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PixelStorei: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PixelTransferf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PixelTransferi: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PixelZoom: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PointSize: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PolygonMode: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PolygonStipple: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PopAttrib: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PopMatrix: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PopName: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PushAttrib: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PushMatrix: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut PushName: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos2d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos2dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos2f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos2fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos2i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos2iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos2s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos2sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos3d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos3dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos3f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos3fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos3i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos3iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos3s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos3sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos4d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos4dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos4f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos4fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos4i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos4iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos4s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RasterPos4sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ReadBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ReadPixels: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Rectd: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Rectdv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Rectf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Rectfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Recti: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Rectiv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Rects: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Rectsv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut RenderMode: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Rotated: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Rotatef: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Scaled: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Scalef: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Scissor: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut SelectBuffer: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut ShadeModel: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut StencilFunc: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut StencilMask: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut StencilOp: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord1d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord1dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord1f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord1fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord1i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord1iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord1s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord1sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord2d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord2dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord2f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord2fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord2i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord2iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord2s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord2sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord3d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord3dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord3f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord3fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord3i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord3iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord3s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord3sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord4d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord4dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord4f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord4fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord4i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord4iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord4s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexCoord4sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexEnvf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexEnvfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexEnvi: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexEnviv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexGend: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexGendv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexGenf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexGenfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexGeni: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexGeniv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexImage1D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexImage2D: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexParameterf: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexParameterfv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexParameteri: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut TexParameteriv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Translated: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Translatef: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex2d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex2dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex2f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex2fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex2i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex2iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex2s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex2sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex3d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex3dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex3f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex3fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex3i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex3iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex3s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex3sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex4d: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex4dv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex4f: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex4fv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex4i: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex4iv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex4s: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Vertex4sv: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
     pub static mut Viewport: FnPtr = FnPtr { ptr: FnPtr::not_initialized as *const raw::c_void, is_loaded: false };
}

pub fn load<F>(mut loadfn: F) where F: FnMut(&'static str) -> *const raw::c_void {
    unsafe {
         storage::Accum.load(&mut loadfn, "glAccum");
         storage::AlphaFunc.load(&mut loadfn, "glAlphaFunc");
         storage::Begin.load(&mut loadfn, "glBegin");
         storage::Bitmap.load(&mut loadfn, "glBitmap");
         storage::BlendFunc.load(&mut loadfn, "glBlendFunc");
         storage::CallList.load(&mut loadfn, "glCallList");
         storage::CallLists.load(&mut loadfn, "glCallLists");
         storage::Clear.load(&mut loadfn, "glClear");
         storage::ClearAccum.load(&mut loadfn, "glClearAccum");
         storage::ClearColor.load(&mut loadfn, "glClearColor");
         storage::ClearDepth.load(&mut loadfn, "glClearDepth");
         storage::ClearIndex.load(&mut loadfn, "glClearIndex");
         storage::ClearStencil.load(&mut loadfn, "glClearStencil");
         storage::ClipPlane.load(&mut loadfn, "glClipPlane");
         storage::Color3b.load(&mut loadfn, "glColor3b");
         storage::Color3bv.load(&mut loadfn, "glColor3bv");
         storage::Color3d.load(&mut loadfn, "glColor3d");
         storage::Color3dv.load(&mut loadfn, "glColor3dv");
         storage::Color3f.load(&mut loadfn, "glColor3f");
         storage::Color3fv.load(&mut loadfn, "glColor3fv");
         storage::Color3i.load(&mut loadfn, "glColor3i");
         storage::Color3iv.load(&mut loadfn, "glColor3iv");
         storage::Color3s.load(&mut loadfn, "glColor3s");
         storage::Color3sv.load(&mut loadfn, "glColor3sv");
         storage::Color3ub.load(&mut loadfn, "glColor3ub");
         storage::Color3ubv.load(&mut loadfn, "glColor3ubv");
         storage::Color3ui.load(&mut loadfn, "glColor3ui");
         storage::Color3uiv.load(&mut loadfn, "glColor3uiv");
         storage::Color3us.load(&mut loadfn, "glColor3us");
         storage::Color3usv.load(&mut loadfn, "glColor3usv");
         storage::Color4b.load(&mut loadfn, "glColor4b");
         storage::Color4bv.load(&mut loadfn, "glColor4bv");
         storage::Color4d.load(&mut loadfn, "glColor4d");
         storage::Color4dv.load(&mut loadfn, "glColor4dv");
         storage::Color4f.load(&mut loadfn, "glColor4f");
         storage::Color4fv.load(&mut loadfn, "glColor4fv");
         storage::Color4i.load(&mut loadfn, "glColor4i");
         storage::Color4iv.load(&mut loadfn, "glColor4iv");
         storage::Color4s.load(&mut loadfn, "glColor4s");
         storage::Color4sv.load(&mut loadfn, "glColor4sv");
         storage::Color4ub.load(&mut loadfn, "glColor4ub");
         storage::Color4ubv.load(&mut loadfn, "glColor4ubv");
         storage::Color4ui.load(&mut loadfn, "glColor4ui");
         storage::Color4uiv.load(&mut loadfn, "glColor4uiv");
         storage::Color4us.load(&mut loadfn, "glColor4us");
         storage::Color4usv.load(&mut loadfn, "glColor4usv");
         storage::ColorMask.load(&mut loadfn, "glColorMask");
         storage::ColorMaterial.load(&mut loadfn, "glColorMaterial");
         storage::CopyPixels.load(&mut loadfn, "glCopyPixels");
         storage::CullFace.load(&mut loadfn, "glCullFace");
         storage::DeleteLists.load(&mut loadfn, "glDeleteLists");
         storage::DepthFunc.load(&mut loadfn, "glDepthFunc");
         storage::DepthMask.load(&mut loadfn, "glDepthMask");
         storage::DepthRange.load(&mut loadfn, "glDepthRange");
         storage::Disable.load(&mut loadfn, "glDisable");
         storage::DrawBuffer.load(&mut loadfn, "glDrawBuffer");
         storage::DrawPixels.load(&mut loadfn, "glDrawPixels");
         storage::EdgeFlag.load(&mut loadfn, "glEdgeFlag");
         storage::EdgeFlagv.load(&mut loadfn, "glEdgeFlagv");
         storage::Enable.load(&mut loadfn, "glEnable");
         storage::End.load(&mut loadfn, "glEnd");
         storage::EndList.load(&mut loadfn, "glEndList");
         storage::EvalCoord1d.load(&mut loadfn, "glEvalCoord1d");
         storage::EvalCoord1dv.load(&mut loadfn, "glEvalCoord1dv");
         storage::EvalCoord1f.load(&mut loadfn, "glEvalCoord1f");
         storage::EvalCoord1fv.load(&mut loadfn, "glEvalCoord1fv");
         storage::EvalCoord2d.load(&mut loadfn, "glEvalCoord2d");
         storage::EvalCoord2dv.load(&mut loadfn, "glEvalCoord2dv");
         storage::EvalCoord2f.load(&mut loadfn, "glEvalCoord2f");
         storage::EvalCoord2fv.load(&mut loadfn, "glEvalCoord2fv");
         storage::EvalMesh1.load(&mut loadfn, "glEvalMesh1");
         storage::EvalMesh2.load(&mut loadfn, "glEvalMesh2");
         storage::EvalPoint1.load(&mut loadfn, "glEvalPoint1");
         storage::EvalPoint2.load(&mut loadfn, "glEvalPoint2");
         storage::FeedbackBuffer.load(&mut loadfn, "glFeedbackBuffer");
         storage::Finish.load(&mut loadfn, "glFinish");
         storage::Flush.load(&mut loadfn, "glFlush");
         storage::Fogf.load(&mut loadfn, "glFogf");
         storage::Fogfv.load(&mut loadfn, "glFogfv");
         storage::Fogi.load(&mut loadfn, "glFogi");
         storage::Fogiv.load(&mut loadfn, "glFogiv");
         storage::FrontFace.load(&mut loadfn, "glFrontFace");
         storage::Frustum.load(&mut loadfn, "glFrustum");
         storage::GenLists.load(&mut loadfn, "glGenLists");
         storage::GetBooleanv.load(&mut loadfn, "glGetBooleanv");
         storage::GetClipPlane.load(&mut loadfn, "glGetClipPlane");
         storage::GetDoublev.load(&mut loadfn, "glGetDoublev");
         storage::GetError.load(&mut loadfn, "glGetError");
         storage::GetFloatv.load(&mut loadfn, "glGetFloatv");
         storage::GetIntegerv.load(&mut loadfn, "glGetIntegerv");
         storage::GetLightfv.load(&mut loadfn, "glGetLightfv");
         storage::GetLightiv.load(&mut loadfn, "glGetLightiv");
         storage::GetMapdv.load(&mut loadfn, "glGetMapdv");
         storage::GetMapfv.load(&mut loadfn, "glGetMapfv");
         storage::GetMapiv.load(&mut loadfn, "glGetMapiv");
         storage::GetMaterialfv.load(&mut loadfn, "glGetMaterialfv");
         storage::GetMaterialiv.load(&mut loadfn, "glGetMaterialiv");
         storage::GetPixelMapfv.load(&mut loadfn, "glGetPixelMapfv");
         storage::GetPixelMapuiv.load(&mut loadfn, "glGetPixelMapuiv");
         storage::GetPixelMapusv.load(&mut loadfn, "glGetPixelMapusv");
         storage::GetPolygonStipple.load(&mut loadfn, "glGetPolygonStipple");
         storage::GetString.load(&mut loadfn, "glGetString");
         storage::GetTexEnvfv.load(&mut loadfn, "glGetTexEnvfv");
         storage::GetTexEnviv.load(&mut loadfn, "glGetTexEnviv");
         storage::GetTexGendv.load(&mut loadfn, "glGetTexGendv");
         storage::GetTexGenfv.load(&mut loadfn, "glGetTexGenfv");
         storage::GetTexGeniv.load(&mut loadfn, "glGetTexGeniv");
         storage::GetTexImage.load(&mut loadfn, "glGetTexImage");
         storage::GetTexLevelParameterfv.load(&mut loadfn, "glGetTexLevelParameterfv");
         storage::GetTexLevelParameteriv.load(&mut loadfn, "glGetTexLevelParameteriv");
         storage::GetTexParameterfv.load(&mut loadfn, "glGetTexParameterfv");
         storage::GetTexParameteriv.load(&mut loadfn, "glGetTexParameteriv");
         storage::Hint.load(&mut loadfn, "glHint");
         storage::IndexMask.load(&mut loadfn, "glIndexMask");
         storage::Indexd.load(&mut loadfn, "glIndexd");
         storage::Indexdv.load(&mut loadfn, "glIndexdv");
         storage::Indexf.load(&mut loadfn, "glIndexf");
         storage::Indexfv.load(&mut loadfn, "glIndexfv");
         storage::Indexi.load(&mut loadfn, "glIndexi");
         storage::Indexiv.load(&mut loadfn, "glIndexiv");
         storage::Indexs.load(&mut loadfn, "glIndexs");
         storage::Indexsv.load(&mut loadfn, "glIndexsv");
         storage::InitNames.load(&mut loadfn, "glInitNames");
         storage::IsEnabled.load(&mut loadfn, "glIsEnabled");
         storage::IsList.load(&mut loadfn, "glIsList");
         storage::LightModelf.load(&mut loadfn, "glLightModelf");
         storage::LightModelfv.load(&mut loadfn, "glLightModelfv");
         storage::LightModeli.load(&mut loadfn, "glLightModeli");
         storage::LightModeliv.load(&mut loadfn, "glLightModeliv");
         storage::Lightf.load(&mut loadfn, "glLightf");
         storage::Lightfv.load(&mut loadfn, "glLightfv");
         storage::Lighti.load(&mut loadfn, "glLighti");
         storage::Lightiv.load(&mut loadfn, "glLightiv");
         storage::LineStipple.load(&mut loadfn, "glLineStipple");
         storage::LineWidth.load(&mut loadfn, "glLineWidth");
         storage::ListBase.load(&mut loadfn, "glListBase");
         storage::LoadIdentity.load(&mut loadfn, "glLoadIdentity");
         storage::LoadMatrixd.load(&mut loadfn, "glLoadMatrixd");
         storage::LoadMatrixf.load(&mut loadfn, "glLoadMatrixf");
         storage::LoadName.load(&mut loadfn, "glLoadName");
         storage::LogicOp.load(&mut loadfn, "glLogicOp");
         storage::Map1d.load(&mut loadfn, "glMap1d");
         storage::Map1f.load(&mut loadfn, "glMap1f");
         storage::Map2d.load(&mut loadfn, "glMap2d");
         storage::Map2f.load(&mut loadfn, "glMap2f");
         storage::MapGrid1d.load(&mut loadfn, "glMapGrid1d");
         storage::MapGrid1f.load(&mut loadfn, "glMapGrid1f");
         storage::MapGrid2d.load(&mut loadfn, "glMapGrid2d");
         storage::MapGrid2f.load(&mut loadfn, "glMapGrid2f");
         storage::Materialf.load(&mut loadfn, "glMaterialf");
         storage::Materialfv.load(&mut loadfn, "glMaterialfv");
         storage::Materiali.load(&mut loadfn, "glMateriali");
         storage::Materialiv.load(&mut loadfn, "glMaterialiv");
         storage::MatrixMode.load(&mut loadfn, "glMatrixMode");
         storage::MultMatrixd.load(&mut loadfn, "glMultMatrixd");
         storage::MultMatrixf.load(&mut loadfn, "glMultMatrixf");
         storage::NewList.load(&mut loadfn, "glNewList");
         storage::Normal3b.load(&mut loadfn, "glNormal3b");
         storage::Normal3bv.load(&mut loadfn, "glNormal3bv");
         storage::Normal3d.load(&mut loadfn, "glNormal3d");
         storage::Normal3dv.load(&mut loadfn, "glNormal3dv");
         storage::Normal3f.load(&mut loadfn, "glNormal3f");
         storage::Normal3fv.load(&mut loadfn, "glNormal3fv");
         storage::Normal3i.load(&mut loadfn, "glNormal3i");
         storage::Normal3iv.load(&mut loadfn, "glNormal3iv");
         storage::Normal3s.load(&mut loadfn, "glNormal3s");
         storage::Normal3sv.load(&mut loadfn, "glNormal3sv");
         storage::Ortho.load(&mut loadfn, "glOrtho");
         storage::PassThrough.load(&mut loadfn, "glPassThrough");
         storage::PixelMapfv.load(&mut loadfn, "glPixelMapfv");
         storage::PixelMapuiv.load(&mut loadfn, "glPixelMapuiv");
         storage::PixelMapusv.load(&mut loadfn, "glPixelMapusv");
         storage::PixelStoref.load(&mut loadfn, "glPixelStoref");
         storage::PixelStorei.load(&mut loadfn, "glPixelStorei");
         storage::PixelTransferf.load(&mut loadfn, "glPixelTransferf");
         storage::PixelTransferi.load(&mut loadfn, "glPixelTransferi");
         storage::PixelZoom.load(&mut loadfn, "glPixelZoom");
         storage::PointSize.load(&mut loadfn, "glPointSize");
         storage::PolygonMode.load(&mut loadfn, "glPolygonMode");
         storage::PolygonStipple.load(&mut loadfn, "glPolygonStipple");
         storage::PopAttrib.load(&mut loadfn, "glPopAttrib");
         storage::PopMatrix.load(&mut loadfn, "glPopMatrix");
         storage::PopName.load(&mut loadfn, "glPopName");
         storage::PushAttrib.load(&mut loadfn, "glPushAttrib");
         storage::PushMatrix.load(&mut loadfn, "glPushMatrix");
         storage::PushName.load(&mut loadfn, "glPushName");
         storage::RasterPos2d.load(&mut loadfn, "glRasterPos2d");
         storage::RasterPos2dv.load(&mut loadfn, "glRasterPos2dv");
         storage::RasterPos2f.load(&mut loadfn, "glRasterPos2f");
         storage::RasterPos2fv.load(&mut loadfn, "glRasterPos2fv");
         storage::RasterPos2i.load(&mut loadfn, "glRasterPos2i");
         storage::RasterPos2iv.load(&mut loadfn, "glRasterPos2iv");
         storage::RasterPos2s.load(&mut loadfn, "glRasterPos2s");
         storage::RasterPos2sv.load(&mut loadfn, "glRasterPos2sv");
         storage::RasterPos3d.load(&mut loadfn, "glRasterPos3d");
         storage::RasterPos3dv.load(&mut loadfn, "glRasterPos3dv");
         storage::RasterPos3f.load(&mut loadfn, "glRasterPos3f");
         storage::RasterPos3fv.load(&mut loadfn, "glRasterPos3fv");
         storage::RasterPos3i.load(&mut loadfn, "glRasterPos3i");
         storage::RasterPos3iv.load(&mut loadfn, "glRasterPos3iv");
         storage::RasterPos3s.load(&mut loadfn, "glRasterPos3s");
         storage::RasterPos3sv.load(&mut loadfn, "glRasterPos3sv");
         storage::RasterPos4d.load(&mut loadfn, "glRasterPos4d");
         storage::RasterPos4dv.load(&mut loadfn, "glRasterPos4dv");
         storage::RasterPos4f.load(&mut loadfn, "glRasterPos4f");
         storage::RasterPos4fv.load(&mut loadfn, "glRasterPos4fv");
         storage::RasterPos4i.load(&mut loadfn, "glRasterPos4i");
         storage::RasterPos4iv.load(&mut loadfn, "glRasterPos4iv");
         storage::RasterPos4s.load(&mut loadfn, "glRasterPos4s");
         storage::RasterPos4sv.load(&mut loadfn, "glRasterPos4sv");
         storage::ReadBuffer.load(&mut loadfn, "glReadBuffer");
         storage::ReadPixels.load(&mut loadfn, "glReadPixels");
         storage::Rectd.load(&mut loadfn, "glRectd");
         storage::Rectdv.load(&mut loadfn, "glRectdv");
         storage::Rectf.load(&mut loadfn, "glRectf");
         storage::Rectfv.load(&mut loadfn, "glRectfv");
         storage::Recti.load(&mut loadfn, "glRecti");
         storage::Rectiv.load(&mut loadfn, "glRectiv");
         storage::Rects.load(&mut loadfn, "glRects");
         storage::Rectsv.load(&mut loadfn, "glRectsv");
         storage::RenderMode.load(&mut loadfn, "glRenderMode");
         storage::Rotated.load(&mut loadfn, "glRotated");
         storage::Rotatef.load(&mut loadfn, "glRotatef");
         storage::Scaled.load(&mut loadfn, "glScaled");
         storage::Scalef.load(&mut loadfn, "glScalef");
         storage::Scissor.load(&mut loadfn, "glScissor");
         storage::SelectBuffer.load(&mut loadfn, "glSelectBuffer");
         storage::ShadeModel.load(&mut loadfn, "glShadeModel");
         storage::StencilFunc.load(&mut loadfn, "glStencilFunc");
         storage::StencilMask.load(&mut loadfn, "glStencilMask");
         storage::StencilOp.load(&mut loadfn, "glStencilOp");
         storage::TexCoord1d.load(&mut loadfn, "glTexCoord1d");
         storage::TexCoord1dv.load(&mut loadfn, "glTexCoord1dv");
         storage::TexCoord1f.load(&mut loadfn, "glTexCoord1f");
         storage::TexCoord1fv.load(&mut loadfn, "glTexCoord1fv");
         storage::TexCoord1i.load(&mut loadfn, "glTexCoord1i");
         storage::TexCoord1iv.load(&mut loadfn, "glTexCoord1iv");
         storage::TexCoord1s.load(&mut loadfn, "glTexCoord1s");
         storage::TexCoord1sv.load(&mut loadfn, "glTexCoord1sv");
         storage::TexCoord2d.load(&mut loadfn, "glTexCoord2d");
         storage::TexCoord2dv.load(&mut loadfn, "glTexCoord2dv");
         storage::TexCoord2f.load(&mut loadfn, "glTexCoord2f");
         storage::TexCoord2fv.load(&mut loadfn, "glTexCoord2fv");
         storage::TexCoord2i.load(&mut loadfn, "glTexCoord2i");
         storage::TexCoord2iv.load(&mut loadfn, "glTexCoord2iv");
         storage::TexCoord2s.load(&mut loadfn, "glTexCoord2s");
         storage::TexCoord2sv.load(&mut loadfn, "glTexCoord2sv");
         storage::TexCoord3d.load(&mut loadfn, "glTexCoord3d");
         storage::TexCoord3dv.load(&mut loadfn, "glTexCoord3dv");
         storage::TexCoord3f.load(&mut loadfn, "glTexCoord3f");
         storage::TexCoord3fv.load(&mut loadfn, "glTexCoord3fv");
         storage::TexCoord3i.load(&mut loadfn, "glTexCoord3i");
         storage::TexCoord3iv.load(&mut loadfn, "glTexCoord3iv");
         storage::TexCoord3s.load(&mut loadfn, "glTexCoord3s");
         storage::TexCoord3sv.load(&mut loadfn, "glTexCoord3sv");
         storage::TexCoord4d.load(&mut loadfn, "glTexCoord4d");
         storage::TexCoord4dv.load(&mut loadfn, "glTexCoord4dv");
         storage::TexCoord4f.load(&mut loadfn, "glTexCoord4f");
         storage::TexCoord4fv.load(&mut loadfn, "glTexCoord4fv");
         storage::TexCoord4i.load(&mut loadfn, "glTexCoord4i");
         storage::TexCoord4iv.load(&mut loadfn, "glTexCoord4iv");
         storage::TexCoord4s.load(&mut loadfn, "glTexCoord4s");
         storage::TexCoord4sv.load(&mut loadfn, "glTexCoord4sv");
         storage::TexEnvf.load(&mut loadfn, "glTexEnvf");
         storage::TexEnvfv.load(&mut loadfn, "glTexEnvfv");
         storage::TexEnvi.load(&mut loadfn, "glTexEnvi");
         storage::TexEnviv.load(&mut loadfn, "glTexEnviv");
         storage::TexGend.load(&mut loadfn, "glTexGend");
         storage::TexGendv.load(&mut loadfn, "glTexGendv");
         storage::TexGenf.load(&mut loadfn, "glTexGenf");
         storage::TexGenfv.load(&mut loadfn, "glTexGenfv");
         storage::TexGeni.load(&mut loadfn, "glTexGeni");
         storage::TexGeniv.load(&mut loadfn, "glTexGeniv");
         storage::TexImage1D.load(&mut loadfn, "glTexImage1D");
         storage::TexImage2D.load(&mut loadfn, "glTexImage2D");
         storage::TexParameterf.load(&mut loadfn, "glTexParameterf");
         storage::TexParameterfv.load(&mut loadfn, "glTexParameterfv");
         storage::TexParameteri.load(&mut loadfn, "glTexParameteri");
         storage::TexParameteriv.load(&mut loadfn, "glTexParameteriv");
         storage::Translated.load(&mut loadfn, "glTranslated");
         storage::Translatef.load(&mut loadfn, "glTranslatef");
         storage::Vertex2d.load(&mut loadfn, "glVertex2d");
         storage::Vertex2dv.load(&mut loadfn, "glVertex2dv");
         storage::Vertex2f.load(&mut loadfn, "glVertex2f");
         storage::Vertex2fv.load(&mut loadfn, "glVertex2fv");
         storage::Vertex2i.load(&mut loadfn, "glVertex2i");
         storage::Vertex2iv.load(&mut loadfn, "glVertex2iv");
         storage::Vertex2s.load(&mut loadfn, "glVertex2s");
         storage::Vertex2sv.load(&mut loadfn, "glVertex2sv");
         storage::Vertex3d.load(&mut loadfn, "glVertex3d");
         storage::Vertex3dv.load(&mut loadfn, "glVertex3dv");
         storage::Vertex3f.load(&mut loadfn, "glVertex3f");
         storage::Vertex3fv.load(&mut loadfn, "glVertex3fv");
         storage::Vertex3i.load(&mut loadfn, "glVertex3i");
         storage::Vertex3iv.load(&mut loadfn, "glVertex3iv");
         storage::Vertex3s.load(&mut loadfn, "glVertex3s");
         storage::Vertex3sv.load(&mut loadfn, "glVertex3sv");
         storage::Vertex4d.load(&mut loadfn, "glVertex4d");
         storage::Vertex4dv.load(&mut loadfn, "glVertex4dv");
         storage::Vertex4f.load(&mut loadfn, "glVertex4f");
         storage::Vertex4fv.load(&mut loadfn, "glVertex4fv");
         storage::Vertex4i.load(&mut loadfn, "glVertex4i");
         storage::Vertex4iv.load(&mut loadfn, "glVertex4iv");
         storage::Vertex4s.load(&mut loadfn, "glVertex4s");
         storage::Vertex4sv.load(&mut loadfn, "glVertex4sv");
         storage::Viewport.load(&mut loadfn, "glViewport");

    }
}

