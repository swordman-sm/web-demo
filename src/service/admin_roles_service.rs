use rbatis::crud::CRUD;
use rbatis::wrapper::Wrapper;
use crate::model::domain::AdminRoles;
use crate::dao::RB;

pub struct AdminRolesService {}

impl AdminRolesService {
    pub async fn get_all_admin_roles_id_name(&self) -> Vec<AdminRoles> {
        // let mut w = Wrapper::new(&RB.driver_type().unwrap())
        //     .set_sql("select id,name from admin_roles");

        let exist_admin_roles: Vec<AdminRoles> = RB.fetch_list().await.unwrap();
        return exist_admin_roles;
        // let exist_admin_roles: Option<Vec<AdminRoles>> = RB.fetch_by_wrapper(w).await?;
        // return match exist_admin_roles {
        //     None => vec![],
        //     Some(v) => v,
        // };
    }
}