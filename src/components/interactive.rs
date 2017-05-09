use specs;

use def::Size;


/// 输入接收组件，作为接收输入的实体的标记。
///
/// 本身无内容，用于指示输入系统操作具备该组件的实体。
pub struct InputReceiver;

impl specs::Component for InputReceiver {
    type Storage = specs::VecStorage<InputReceiver>;
}
