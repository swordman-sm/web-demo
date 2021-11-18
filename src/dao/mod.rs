use lazy_static::lazy_static;
use rbatis::plugin::logic_delete::RbatisLogicDeletePlugin;
use rbatis::rbatis::Rbatis;
lazy_static! {
    pub static ref RB:Rbatis = {
        let mut rb =  Rbatis::new();
        let del = RbatisLogicDeletePlugin::new("del_state");
        rb.logic_plugin = Some(Box::new(del))
        rb
    };
}