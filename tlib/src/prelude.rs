pub use macros::{extends_object, extends_element};

pub use crate::namespace::{KeyCode, KeyboardModifier};
pub use crate::object::{Object, ObjectExt, ObjectImplExt, ObjectOperation};
pub use crate::types::{IsA, ObjectType, StaticType};
pub use crate::values::{ToValue, Value};
pub use crate::actions::{ActionHubExt, Action, ACTION_HUB};
pub use crate::events::{Event, EventType, KeyEvent};