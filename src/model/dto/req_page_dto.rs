/// 资源分页DTO
#[derive(Serialize, Deserialize, Validate, Clone, Debug)]
pub struct ReqPageDTO {
    #[validate(required)]
    pub page: Option<u64>,
    #[validate(required)]
    pub size: Option<u64>,
    pub user_no: Option<String>,
}