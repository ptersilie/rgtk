// This file is part of rgtk.
//
// rgtk is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rgtk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with rgtk.  If not, see <http://www.gnu.org/licenses/>.

//! A frame that constrains its child to a particular aspect ratio

use libc::c_float;
use std::ptr;
use std::ffi::CString;

use gtk::cast::GTK_ASPECTFRAME;
use gtk::{self, ffi};
use glib::to_gboolean;

/// AspectFrame — A frame that constrains its child to a particular aspect ratio
struct_Widget!(AspectFrame);

impl AspectFrame {
    pub fn new(label: Option<&str>, x_align: f32, y_align: f32, ratio: f32, obey_child: bool) -> Option<AspectFrame> {
        let tmp_pointer = match label {
            Some(l) => unsafe {
                let c_str = CString::from_slice(l.as_bytes());

                ffi::gtk_aspect_frame_new(c_str.as_ptr(), x_align as c_float, y_align as c_float, ratio as c_float, to_gboolean(obey_child))
            },
            None => unsafe { ffi::gtk_aspect_frame_new(ptr::null(), x_align as c_float, y_align as c_float, ratio as c_float, to_gboolean(obey_child)) }
        };
        check_pointer!(tmp_pointer, AspectFrame)
    }

    pub fn set(&mut self,
               x_align: f32,
               y_align: f32,
               ratio: f32,
               obey_child: bool) -> () {

        unsafe {
            ffi::gtk_aspect_frame_set(GTK_ASPECTFRAME(self.pointer), x_align as c_float, y_align as c_float, ratio as c_float, to_gboolean(obey_child));
        }
    }
}

impl_drop!(AspectFrame);
impl_TraitWidget!(AspectFrame);

impl gtk::FrameTrait for AspectFrame {}
impl gtk::ContainerTrait for AspectFrame {}

impl_widget_events!(AspectFrame);
