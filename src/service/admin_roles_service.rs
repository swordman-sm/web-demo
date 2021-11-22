use rbatis::crud::CRUD;
use rbatis::wrapper::Wrapper;
use crate::constant::enums::Result;
use crate::constant::Error;
use crate::model::domain::AdminRoles;
use crate::dao::RB;

pub struct AdminRolesService {}

impl AdminRolesService {
    pub async fn get_all_admin_roles_id_name(&self) -> Result<Vec<AdminRoles>> {
        let mut w = Wrapper::new(&RB.driver_type()?)
            .set_sql("select id,name from admin_roles");
        let exist_admin_roles: Option<Vec<AdminRoles>> = RB.fetch_by_wrapper(w);
        if exist_admin_roles.is_none() {
            return Err(Error::from(format!(
                "用户:{} 不存在!",
                arg.username.as_ref().unwrap()
            )));
        }
        return Ok(exist_admin_roles.unwrap());
    }
}