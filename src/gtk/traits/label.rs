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

use libc::{c_int, c_double};
use std::ffi::CString;

use gtk::{self, ffi};
use glib::{to_bool, to_gboolean};
use gtk::Justification;
use gtk::cast::GTK_LABEL;

pub trait LabelTrait: gtk::WidgetTrait {
    fn set_label(&mut self, text: &str) -> () {
        unsafe {
            let c_str = CString::from_slice(text.as_bytes());

            ffi::gtk_label_set_label(GTK_LABEL(self.unwrap_widget()), c_str.as_ptr())
        }
    }

    fn set_text(&mut self, text: &str) -> () {
        unsafe {
            let c_str = CString::from_slice(text.as_bytes());

	    ffi::gtk_label_set_text(GTK_LABEL(self.unwrap_widget()), c_str.as_ptr())
        }
    }

    fn set_justify(&mut self, jtype: Justification) -> () {
        unsafe {
            ffi::gtk_label_set_justify(GTK_LABEL(self.unwrap_widget()), jtype);
        }
    }

    fn set_markup(&mut self, text: &str) -> () {
        unsafe {
            let c_str = CString::from_slice(text.as_bytes());

            ffi::gtk_label_set_markup(GTK_LABEL(self.unwrap_widget()), c_str.as_ptr())
        }
    }

    fn set_markup_with_mnemonic(&mut self, text: &str) -> () {
        unsafe {
            let c_str = CString::from_slice(text.as_bytes());

            ffi::gtk_label_set_markup_with_mnemonic(GTK_LABEL(self.unwrap_widget()), c_str.as_ptr())
        }
    }

    fn set_pattern(&mut self, text: &str) -> () {
        unsafe {
            let c_str = CString::from_slice(text.as_bytes());

            ffi::gtk_label_set_pattern(GTK_LABEL(self.unwrap_widget()), c_str.as_ptr())
        }
    }

    fn set_text_with_mnemonic(&mut self, text: &str) -> () {
        unsafe {
            let c_str = CString::from_slice(text.as_bytes());

            ffi::gtk_label_set_text_with_mnemonic(GTK_LABEL(self.unwrap_widget()), c_str.as_ptr());
        }
    }

    fn set_width_chars(&mut self, n_chars: i32) -> () {
        unsafe {
            ffi::gtk_label_set_width_chars(GTK_LABEL(self.unwrap_widget()), n_chars as c_int);
        }
    }

    fn set_max_width_chars(&mut self, n_chars: i32) -> () {
        unsafe {
            ffi::gtk_label_set_max_width_chars(GTK_LABEL(self.unwrap_widget()), n_chars as c_int);
        }
    }

    fn set_line_wrap(&mut self, wrap: bool) -> () {
        unsafe { ffi::gtk_label_set_line_wrap(GTK_LABEL(self.unwrap_widget()), to_gboolean(wrap)); }
    }

    fn get_line_wrap(&self) -> bool {
        unsafe { to_bool(ffi::gtk_label_get_line_wrap(GTK_LABEL(self.unwrap_widget()))) }
    }

    #[cfg(any(feature = "GTK_3_10",feature = "GTK_3_12", feature = "GTK_3_14"))]
    fn set_lines(&mut self, lines: i32) -> () {
        unsafe {
            ffi::gtk_label_set_lines(GTK_LABEL(self.unwrap_widget()), lines as c_int);
        }
    }

    #[cfg(any(feature = "GTK_3_10",feature = "GTK_3_12", feature = "GTK_3_14"))]
    fn get_lines(&self) -> i32 {
        unsafe {
            ffi::gtk_label_get_lines(GTK_LABEL(self.unwrap_widget())) as c_int
        }
    }

    fn get_layout_offsets(&self) -> (i32, i32) {
        let x = 0;
        let y = 0;
        unsafe {
            ffi::gtk_label_get_layout_offsets(GTK_LABEL(self.unwrap_widget()), &x, &y);
        }
        (x, y)
    }

    fn get_mnemonic_keyval(&self) -> u32 {
        unsafe {
            ffi::gtk_label_get_mnemonic_keyval(GTK_LABEL(self.unwrap_widget())) as u32
        }
    }

    fn set_selectable(&mut self, selectable: bool) -> () {
        unsafe { ffi::gtk_label_set_selectable(GTK_LABEL(self.unwrap_widget()), to_gboolean(selectable)); }
    }

    fn get_selectable(&self) -> bool {
        unsafe { to_bool(ffi::gtk_label_get_selectable(GTK_LABEL(self.unwrap_widget()))) }
    }

    fn set_use_markup(&mut self, use_markup: bool) -> () {
        unsafe { ffi::gtk_label_set_use_markup(GTK_LABEL(self.unwrap_widget()), to_gboolean(use_markup)); }
    }

    fn get_use_markup(&self) -> bool {
        unsafe { to_bool(ffi::gtk_label_get_use_markup(GTK_LABEL(self.unwrap_widget()))) }
    }

    fn set_use_underline(&mut self, use_underline: bool) -> () {
        unsafe { ffi::gtk_label_set_use_underline(GTK_LABEL(self.unwrap_widget()), to_gboolean(use_underline)); }
    }

    fn get_use_underline(&self) -> bool {
        unsafe { to_bool(ffi::gtk_label_get_use_underline(GTK_LABEL(self.unwrap_widget()))) }
    }

    fn set_single_line_mode(&mut self, single_line_mode: bool) -> () {
        unsafe { ffi::gtk_label_set_single_line_mode(GTK_LABEL(self.unwrap_widget()), to_gboolean(single_line_mode)); }
    }

    fn get_single_line_mode(&self) -> bool {
        unsafe { to_bool(ffi::gtk_label_get_single_line_mode(GTK_LABEL(self.unwrap_widget()))) }
    }

    fn set_track_visited_links(&mut self, track_visited_links: bool) -> () {
        unsafe { ffi::gtk_label_set_track_visited_links(GTK_LABEL(self.unwrap_widget()), to_gboolean(track_visited_links)); }
    }

    fn get_track_visited_links(&self) -> bool {
        unsafe { to_bool(ffi::gtk_label_get_track_visited_links(GTK_LABEL(self.unwrap_widget()))) }
    }

    fn get_text(&self) -> Option<String> {
        unsafe {
            let c_str = ffi::gtk_label_get_text(GTK_LABEL(self.unwrap_widget()));

            if c_str.is_null() {
                None
            } else {
                Some(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&c_str)).to_string())
            }
        }
    }

    fn get_label(&self) -> Option<String> {
        unsafe {
            let c_str = ffi::gtk_label_get_label(GTK_LABEL(self.unwrap_widget()));
            if c_str.is_null() {
                None
            } else {
                Some(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&c_str)).to_string())
            }
        }
    }

    fn get_current_uri(&self) -> Option<String> {
        unsafe {
            let c_str = ffi::gtk_label_get_current_uri(GTK_LABEL(self.unwrap_widget()));
            if c_str.is_null() {
                None
            } else {
                Some(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&c_str)).to_string())
            }
        }
    }

    fn select_region(&mut self, start_offset: i32, end_offset: i32) -> () {
        unsafe {
            ffi::gtk_label_select_region(GTK_LABEL(self.unwrap_widget()), start_offset as c_int, end_offset as c_int);
        }
    }

    fn get_justify(&self) -> Justification {
        unsafe {
            ffi::gtk_label_get_justify(GTK_LABEL(self.unwrap_widget()))
        }
    }

    fn get_width_chars(&self) -> i32 {
        unsafe {
            ffi::gtk_label_get_width_chars(GTK_LABEL(self.unwrap_widget())) as i32
        }
    }

    fn get_max_width_chars(&self) -> i32 {
        unsafe {
            ffi::gtk_label_get_max_width_chars(GTK_LABEL(self.unwrap_widget())) as i32
        }
    }

    fn get_selection_bounds(&self) -> (i32, i32) {
        let x = 0;
        let y = 0;
        unsafe {
            ffi::gtk_label_get_selection_bounds(GTK_LABEL(self.unwrap_widget()), &x, &y);
        }
        (x, y)
    }

    fn set_angle(&mut self, angle: f64) -> () {
        unsafe {
            ffi::gtk_label_set_angle(GTK_LABEL(self.unwrap_widget()), angle as c_double);
        }
    }

    fn get_angle(&self) -> f64 {
        unsafe {
            ffi::gtk_label_get_angle(GTK_LABEL(self.unwrap_widget())) as f64
        }
    }
}
