use crate::WasmEnv;
use wasmer::*;

mod defaults;
mod env;
mod html;
mod js;
mod net;
mod std;

pub fn generate_imports(store: &mut Store, env: &FunctionEnv<WasmEnv>) -> Imports {
	imports! {
		"env" => {
			"abort" => Function::new_typed_with_env(store, env, env::abort),
			"print" => Function::new_typed_with_env(store, env, env::print),
			"sleep" => Function::new_typed_with_env(store, env, env::sleep),
			"send_partial_result" => Function::new_typed_with_env(store, env, env::send_partial_result),
		},
		"std" => {
			"destroy" => Function::new_typed_with_env(store, env, std::destroy),
			"buffer_len" => Function::new_typed_with_env(store, env, std::buffer_len),
			"read_buffer" => Function::new_typed_with_env(store, env, std::read_buffer),
			"current_date" => Function::new_typed_with_env(store, env, std::current_date),
			"utc_offset" => Function::new_typed_with_env(store, env, std::utc_offset),
			"parse_date" => Function::new_typed_with_env(store, env, std::parse_date),
		},
		"defaults" => {
			"get" => Function::new_typed_with_env(store, env, defaults::get),
			"set" => Function::new_typed_with_env(store, env, defaults::set),
		},
		"html" => {
			"parse" => Function::new_typed_with_env(store, env, html::parse),
			"parse_fragment" => Function::new_typed_with_env(store, env, html::parse_fragment),
			"escape" => Function::new_typed_with_env(store, env, html::escape),
			"unescape" => Function::new_typed_with_env(store, env, html::unescape),

			"select" => Function::new_typed_with_env(store, env, html::select),
			"select_first" => Function::new_typed_with_env(store, env, html::select_first),
			"attr" => Function::new_typed_with_env(store, env, html::attr),
			"text" => Function::new_typed_with_env(store, env, html::text),
			"untrimmed_text" => Function::new_typed_with_env(store, env, html::untrimmed_text),
			"html" => Function::new_typed_with_env(store, env, html::html),
			"outer_html" => Function::new_typed_with_env(store, env, html::outer_html),

			"set_text" => Function::new_typed_with_env(store, env, html::set_text),
			"set_html" => Function::new_typed_with_env(store, env, html::set_html),
			"prepend" => Function::new_typed_with_env(store, env, html::prepend),
			"append" => Function::new_typed_with_env(store, env, html::append),
			"parent" => Function::new_typed_with_env(store, env, html::parent),
			"children" => Function::new_typed_with_env(store, env, html::children),
			"siblings" => Function::new_typed_with_env(store, env, html::siblings),
			"next" => Function::new_typed_with_env(store, env, html::next),
			"previous" => Function::new_typed_with_env(store, env, html::previous),
			"base_uri" => Function::new_typed_with_env(store, env, html::base_uri),
			"own_text" => Function::new_typed_with_env(store, env, html::own_text),
			"data" => Function::new_typed_with_env(store, env, html::data),
			"id" => Function::new_typed_with_env(store, env, html::id),
			"tag_name" => Function::new_typed_with_env(store, env, html::tag_name),
			"class_name" => Function::new_typed_with_env(store, env, html::class_name),
			"has_class" => Function::new_typed_with_env(store, env, html::has_class),
			"has_attr" => Function::new_typed_with_env(store, env, html::has_attr),

			"first" => Function::new_typed_with_env(store, env, html::first),
			"last" => Function::new_typed_with_env(store, env, html::last),
			"get" => Function::new_typed_with_env(store, env, html::get),
			"size" => Function::new_typed_with_env(store, env, html::size),
		},
		"js" => {
			"context_create" => Function::new_typed_with_env(store, env, js::context_create),
			"context_eval" => Function::new_typed_with_env(store, env, js::context_eval),
			"context_get" => Function::new_typed_with_env(store, env, js::context_get),

			"webview_create" => Function::new_typed_with_env(store, env, js::webview_create),
			"webview_load" => Function::new_typed_with_env(store, env, js::webview_load),
			"webview_load_html" => Function::new_typed_with_env(store, env, js::webview_load_html),
			"webview_wait_for_load" => Function::new_typed_with_env(store, env, js::webview_wait_for_load),
			"webview_eval" => Function::new_typed_with_env(store, env, js::webview_eval),
		},
		"net" => {
			"init" => Function::new_typed_with_env(store, env, net::init),
			"send" => Function::new_typed_with_env(store, env, net::send),
			"send_all" => Function::new_typed_with_env(store, env, net::send_all),

			"set_url" => Function::new_typed_with_env(store, env, net::set_url),
			"set_header" => Function::new_typed_with_env(store, env, net::set_header),
			"set_body" => Function::new_typed_with_env(store, env, net::set_body),

			"data_len" => Function::new_typed_with_env(store, env, net::data_len),
			"read_data" => Function::new_typed_with_env(store, env, net::read_data),
			"get_image" => Function::new_typed_with_env(store, env, net::get_image),
			"get_status_code" => Function::new_typed_with_env(store, env, net::get_status_code),
			"get_header" => Function::new_typed_with_env(store, env, net::get_header),
			"html" => Function::new_typed_with_env(store, env, net::html),

			"set_rate_limit" => Function::new_typed_with_env(store, env, net::set_rate_limit),
		},
	}
}
