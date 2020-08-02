//! # glu-sys
//! Raw GLU and GL Rust bindings

//! This crate doesn't handle windowing, it can be used with other crates which handle windowing and gl contexts to do raw opengl calls.

//! ```rust
//! fn draw_triangle() {
//!     use glu_sys::*;
//!     unsafe {
//!         glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
//!         glMatrixMode(GL_PROJECTION);
//!         glLoadIdentity();
//!         glViewport(0, 0, W, H);
//!         gluPerspective(45.0, (W as f32 / H as f32).into(), 1.0, 10.0);
//!         glTranslatef(0.0, 0.0, -5.0);
//!         glMatrixMode(GL_MODELVIEW);
//!         glLoadIdentity();
//!         glRotatef(0.0, 1.0, 1.0, 0.0);
//!         glColor3f(1.0, 0.0, 0.0);
//!         glBegin(GL_POLYGON);
//!         glVertex3f(0.0, 1.0, 0.0);
//!         glVertex3f(1.0, -1.0, 1.0);
//!         glVertex3f(-1.0, -1.0, 1.0);
//!         glEnd();
//!         glColor3f(0.0, 1.0, 0.0);
//!         glBegin(GL_POLYGON);
//!         glVertex3f(0.0, 1.0, 0.0);
//!         glVertex3f(0.0, -1.0, -1.0);
//!         glVertex3f(1.0, -1.0, 1.0);
//!         glEnd();
//!         glColor3f(0.0, 0.0, 1.0);
//!         glBegin(GL_POLYGON);
//!         glVertex3f(0.0, 1.0, 0.0);
//!         glVertex3f(-1.0, -1.0, 1.0);
//!         glVertex3f(0.0, -1.0, -1.0);
//!         glEnd();
//!         glColor3f(1.0, 0.0, 0.0);
//!         glBegin(GL_POLYGON);
//!         glVertex3f(1.0, -1.0, 1.0);
//!         glVertex3f(0.0, -1.0, -1.0);
//!         glVertex3f(-1.0, -1.0, 1.0);
//!         glEnd();
//!         glLoadIdentity();
//!         glRasterPos2f(-3.0, -2.0);
//!     }
//! }
//! ```

//! Full example using the fltk crate [here](https://github.com/MoAlyousef/fltk-rs-demos/blob/master/opengl/src/main.rs).

#![no_std]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(non_upper_case_globals)]

pub mod glu;

pub use glu::*;