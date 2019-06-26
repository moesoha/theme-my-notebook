#![recursion_limit="128"]

#[macro_use]
extern crate sohablog_lib;

use sohablog_lib::{
	plugin::{THEME_TRAIT_VERSION, Theme, PluginMetadata, PluginType},
	utils::{Page, TemplateContext, StaticFile},
	interfaces::models::{
		Content,
		Author
	},
};
use std::io::{Result, Write};

include!(concat!(env!("OUT_DIR"), "/templates/templates.rs"));
impl StaticFile for &templates::statics::StaticFile {
	fn content(&self) -> &'static [u8] { self.content }
	fn name(&self) -> &'static str { self.name }
	fn mime(&self) -> &'static mime::Mime { self.mime }
}

#[derive(Debug, Default)]
pub struct MyNotebook;
impl PluginMetadata for MyNotebook {
	fn plugin_version(&self) -> u32 { THEME_TRAIT_VERSION }
	fn name(&self) -> &'static str { "My Notebook" }
	fn description(&self) -> &'static str { "Designed by Soha Jin. https://github.com/moesoha/my-notebook" }
	fn version(&self) -> &'static str { env!("CARGO_PKG_VERSION") }
	fn r#type(&self) -> PluginType { PluginType::Theme }
}
impl Theme for MyNotebook {
	fn identity(&self) -> &'static str { "my-notebook" }
	fn post_list(&self, out: &mut Write, ctx: &TemplateContext, title: &str, page: Page, posts: Vec<Box<Content>>) -> Result<()> {
		templates::post_list(out, ctx, title, page, posts)?;
		Ok(())
	}
	fn post_show(&self, out: &mut Write, ctx: &TemplateContext, title: &str, post: Box<Content>, previous_author: Option<Box<Author>>) -> Result<()> {
		templates::post_show(out, ctx, title, post, previous_author)?;
		Ok(())
	}
	fn static_file(&self, name: &str) -> Option<Box<StaticFile>> {
		if let Some(f) = templates::statics::StaticFile::get(name) {
			Some(Box::new(f) as Box<StaticFile>)
		} else {
			None
		}
	}
}

declare_plugin_metadata!(MyNotebook, MyNotebook::default);
declare_plugin!(MyNotebook, MyNotebook::default, Theme);
