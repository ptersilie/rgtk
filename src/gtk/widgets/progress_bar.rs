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

//! A widget which indicates progress visually

use libc::c_double;
use std::ffi::CString;

use gtk::{self, ffi};
use glib::{to_bool, to_gboolean};
use gtk::cast::GTK_PROGRESSBAR;

/// ProgressBar — A widget which indicates progress visually
struct_Widget!(ProgressBar);

impl ProgressBar {
    pub fn new() -> Option<ProgressBar> {
        let tmp_pointer = unsafe { ffi::gtk_progress_bar_new() };
        check_pointer!(tmp_pointer, ProgressBar)
    }

    pub fn pulse(&mut self) -> () {
        unsafe {
            ffi::gtk_progress_bar_pulse(GTK_PROGRESSBAR(self.pointer))
        }
    }

    pub fn set_fraction(&mut self, fraction: f64) -> () {
        unsafe {
            ffi::gtk_progress_bar_set_fraction(GTK_PROGRESSBAR(self.pointer), fraction as c_double)
        }
    }

    pub fn get_fraction(&self) -> f64 {
        unsafe {
            ffi::gtk_progress_bar_get_fraction(GTK_PROGRESSBAR(self.pointer)) as f64
        }
    }

    pub fn set_text(&mut self, text: &str) -> () {
        unsafe {
            let c_str = CString::from_slice(text.as_bytes());

            ffi::gtk_progress_bar_set_text(GTK_PROGRESSBAR(self.pointer), c_str.as_ptr());
        }
    }

    pub fn get_text(&self) -> Option<String> {
        unsafe {
            let c_str = ffi::gtk_progress_bar_get_text(GTK_PROGRESSBAR(self.pointer));

            if c_str.is_null() {
                None
            } else {
                Some(String::from_utf8_lossy(::std::ffi::c_str_to_bytes(&c_str)).to_string())
            }
        }
    }

    pub fn set_inverted(&mut self, inverted: bool) -> () {
        unsafe { ffi::gtk_progress_bar_set_inverted(GTK_PROGRESSBAR(self.pointer), to_gboolean(inverted)); }
    }

    pub fn get_inverted(&self) -> bool {
        unsafe { to_bool(ffi::gtk_progress_bar_get_inverted(GTK_PROGRESSBAR(self.pointer))) }
    }

    pub fn set_show_text(&mut self, show_text: bool) -> () {
        unsafe { ffi::gtk_progress_bar_set_show_text(GTK_PROGRESSBAR(self.pointer), to_gboolean(show_text)); }
    }

    pub fn get_show_text(&self) -> bool {
        unsafe { to_bool(ffi::gtk_progress_bar_get_show_text(GTK_PROGRESSBAR(self.pointer))) }
    }

    pub fn set_pulse_step(&mut self, pulse_step: f64) -> () {
        unsafe {
            ffi::gtk_progress_bar_set_pulse_step(GTK_PROGRESSBAR(self.pointer), pulse_step as c_double)
        }
    }

    pub fn get_pulse_step(&self) -> f64 {
        unsafe {
            ffi::gtk_progress_bar_get_pulse_step(GTK_PROGRESSBAR(self.pointer)) as f64
        }
    }
}

impl_drop!(ProgressBar);
impl_TraitWidget!(ProgressBar);

impl gtk::OrientableTrait for ProgressBar {}

impl_widget_events!(ProgressBar);