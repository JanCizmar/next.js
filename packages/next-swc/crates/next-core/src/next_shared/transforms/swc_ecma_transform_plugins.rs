use anyhow::Result;
use turbo_binding::turbopack::{
    ecmascript::{OptionTransformPluginVc, TransformPluginVc},
    ecmascript_plugin::transform::swc_ecma_transform_plugins::SwcEcmaTransformPluginsTransformer,
};
use turbo_tasks_fs::FileSystemPathVc;

use crate::next_config::NextConfigVc;

#[turbo_tasks::function]
pub async fn get_swc_ecma_transform_plugin(
    project_path: FileSystemPathVc,
    next_config: NextConfigVc,
) -> Result<OptionTransformPluginVc> {
    let transform_plugin = next_config
        .await?
        .experimental
        .swc_plugins
        .as_ref()
        .map(|value| {
            if value.is_empty() {
                OptionTransformPluginVc::cell(None)
            } else {
                OptionTransformPluginVc::cell(Some(TransformPluginVc::cell(Box::new(
                    SwcEcmaTransformPluginsTransformer::new(&project_path, value),
                ))))
            }
        })
        .unwrap_or_default();

    Ok(transform_plugin)
}
