use glib::{subclass::prelude::*, Object};

use crate::{subclass::channel::ChannelImpl, PortChannel};

pub trait PortChannelImpl: ObjectImpl + ChannelImpl {}

unsafe impl<T: PortChannelImpl> IsSubclassable<T> for PortChannel {
    fn class_init(class: &mut glib::Class<Self>) {
        <Object as IsSubclassable<T>>::class_init(class);
    }

    fn instance_init(instance: &mut glib::subclass::InitializingObject<T>) {
        <Object as IsSubclassable<T>>::instance_init(instance);
    }
}
