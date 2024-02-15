use glib::{subclass::prelude::*, Object};

use crate::Channel;

pub trait ChannelImpl: ObjectImpl {}

unsafe impl<T: ChannelImpl> IsSubclassable<T> for Channel {
    fn class_init(class: &mut glib::Class<Self>) {
        <Object as IsSubclassable<T>>::class_init(class);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <Object as IsSubclassable<T>>::instance_init(instance);
    }
}
