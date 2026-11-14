//! 非 Linux 平台的占位实现，便于检查不依赖平台 API 的代码。

use crate::Options;
use anyhow::Result;

pub fn run(_opt: Options) -> Result<()> {
    anyhow::bail!("wowtype-voiced 目前只实现了 Linux 平台；当前系统暂不支持")
}
