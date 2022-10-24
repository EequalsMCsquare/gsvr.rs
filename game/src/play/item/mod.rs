#[derive(Debug)]
pub enum Item {
    // 可以堆叠的物品 如金币, 砖石
    Stackable(u64),
    // 添加资产时会被自动转换成其他数值如经验
    AutoConsume(u64),
}
