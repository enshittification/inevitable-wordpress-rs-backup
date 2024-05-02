use rstest::*;
use rstest_reuse::{self, apply, template};
use wp_api::{
    SparseUser, SparseUserField, UserListParams, WPApiParamOrder, WPApiParamUsersOrderBy,
    WPApiParamUsersWho, WPContext,
};

use crate::test_helpers::{
    api, WPNetworkRequestExecutor, WPNetworkResponseParser, FIRST_USER_ID, SECOND_USER_ID,
};

pub mod test_helpers;

#[apply(filter_fields_cases)]
#[tokio::test]
async fn filter_users(#[case] fields: &[SparseUserField]) {
    let parsed_response = api()
        .filter_list_users_request(WPContext::Edit, &None, fields)
        .execute()
        .await
        .unwrap()
        .parse(wp_api::parse_filter_users_response);
    assert!(parsed_response.is_ok());
    parsed_response
        .unwrap()
        .iter()
        .for_each(|user| validate_sparse_user_fields(&user, fields));
}

#[apply(filter_fields_cases)]
#[tokio::test]
async fn filter_retrieve_user(#[case] fields: &[SparseUserField]) {
    let user_result = api()
        .filter_retrieve_user_request(FIRST_USER_ID, WPContext::Edit, fields)
        .execute()
        .await
        .unwrap()
        .parse(wp_api::parse_filter_retrieve_user_response);
    assert!(user_result.is_ok());
    validate_sparse_user_fields(&user_result.unwrap(), fields);
}

#[apply(filter_fields_cases)]
#[tokio::test]
async fn filter_retrieve_current_user(#[case] fields: &[SparseUserField]) {
    let user_result = api()
        .filter_retrieve_current_user_request(WPContext::Edit, fields)
        .execute()
        .await
        .unwrap()
        .parse(wp_api::parse_filter_retrieve_user_response);
    assert!(user_result.is_ok());
    validate_sparse_user_fields(&user_result.unwrap(), fields);
}

#[tokio::test]
async fn list_users_with_edit_context() {
    assert!(api()
        .list_users_request(WPContext::Edit, &None)
        .execute()
        .await
        .unwrap()
        .parse(wp_api::parse_list_users_response_with_edit_context)
        .is_ok());
}

#[tokio::test]
async fn list_users_with_embed_context() {
    assert!(api()
        .list_users_request(WPContext::Embed, &None)
        .execute()
        .await
        .unwrap()
        .parse(wp_api::parse_list_users_response_with_embed_context)
        .is_ok());
}

#[tokio::test]
async fn list_users_with_view_context() {
    assert!(api()
        .list_users_request(WPContext::View, &None)
        .execute()
        .await
        .unwrap()
        .parse(wp_api::parse_list_users_response_with_view_context)
        .is_ok());
}

#[tokio::test]
async fn list_users_param_page() {
    let mut params = UserListParams::default();
    params.page = Some(2);
    test_user_list_params(params).await;
}

#[tokio::test]
async fn list_users_param_per_page() {
    let mut params = UserListParams::default();
    params.per_page = Some(2);
    test_user_list_params(params).await;
}

#[tokio::test]
async fn list_users_param_search() {
    let mut params = UserListParams::default();
    params.search = Some("foo".to_string());
    test_user_list_params(params).await;
}

#[tokio::test]
async fn list_users_param_exclude() {
    let mut params = UserListParams::default();
    params.exclude = vec![FIRST_USER_ID];
    test_user_list_params(params).await;
}

#[tokio::test]
async fn list_users_param_include() {
    let mut params = UserListParams::default();
    params.include = vec![SECOND_USER_ID];
    test_user_list_params(params).await;
}

#[tokio::test]
async fn list_users_param_offset() {
    let mut params = UserListParams::default();
    params.offset = Some(2);
    test_user_list_params(params).await;
}

#[tokio::test]
async fn list_users_param_order_asc() {
    let mut params = UserListParams::default();
    params.order = Some(WPApiParamOrder::Asc);
    test_user_list_params(params).await;
}

#[tokio::test]
async fn list_users_param_order_desc() {
    let mut params = UserListParams::default();
    params.order = Some(WPApiParamOrder::Desc);
    test_user_list_params(params).await;
}

#[tokio::test]
async fn list_users_param_orderby_id() {
    let mut params = UserListParams::default();
    params.orderby = Some(WPApiParamUsersOrderBy::Id);
    test_user_list_params(params).await;
}

#[tokio::test]
async fn list_users_param_orderby_include() {
    let mut params = UserListParams::default();
    params.orderby = Some(WPApiParamUsersOrderBy::Include);
    test_user_list_params(params).await;
}

#[tokio::test]
async fn list_users_param_orderby_name() {
    let mut params = UserListParams::default();
    params.orderby = Some(WPApiParamUsersOrderBy::Name);
    test_user_list_params(params).await;
}

#[tokio::test]
async fn list_users_param_orderby_registered_date() {
    let mut params = UserListParams::default();
    params.orderby = Some(WPApiParamUsersOrderBy::RegisteredDate);
    test_user_list_params(params).await;
}

#[tokio::test]
async fn list_users_param_orderby_slug() {
    let mut params = UserListParams::default();
    params.orderby = Some(WPApiParamUsersOrderBy::Slug);
    test_user_list_params(params).await;
}

#[tokio::test]
async fn list_users_param_orderby_include_slugs() {
    let mut params = UserListParams::default();
    params.orderby = Some(WPApiParamUsersOrderBy::IncludeSlugs);
    test_user_list_params(params).await;
}

#[tokio::test]
async fn list_users_param_orderby_email() {
    let mut params = UserListParams::default();
    params.orderby = Some(WPApiParamUsersOrderBy::Email);
    test_user_list_params(params).await;
}

#[tokio::test]
async fn list_users_param_orderby_url() {
    let mut params = UserListParams::default();
    params.orderby = Some(WPApiParamUsersOrderBy::Url);
    test_user_list_params(params).await;
}

#[tokio::test]
async fn list_users_param_slug() {
    let mut params = UserListParams::default();
    params.slug = vec!["foo".to_string()];
    test_user_list_params(params).await;
}

#[tokio::test]
async fn list_users_param_roles_edit_posts() {
    let mut params = UserListParams::default();
    params.roles = vec!["edit_posts".to_string()];
    test_user_list_params(params).await;
}

#[tokio::test]
async fn list_users_param_capabilities() {
    let mut params = UserListParams::default();
    params.capabilities = vec!["foo".to_string()];
    test_user_list_params(params).await;
}

#[tokio::test]
async fn list_users_param_who_all() {
    let mut params = UserListParams::default();
    params.who = Some(WPApiParamUsersWho::All);
    test_user_list_params(params).await;
}

#[tokio::test]
async fn list_users_param_who_authors() {
    let mut params = UserListParams::default();
    params.who = Some(WPApiParamUsersWho::Authors);
    test_user_list_params(params).await;
}

#[tokio::test]
async fn list_users_param_has_published_posts() {
    let mut params = UserListParams::default();
    params.has_published_posts = Some(true);
    test_user_list_params(params).await;
}

#[tokio::test]
async fn retrieve_user_with_edit_context() {
    assert!(api()
        .retrieve_user_request(FIRST_USER_ID, WPContext::Edit)
        .execute()
        .await
        .unwrap()
        .parse(wp_api::parse_retrieve_user_response_with_edit_context)
        .is_ok());
}

#[tokio::test]
async fn retrieve_user_with_embed_context() {
    assert!(api()
        .retrieve_user_request(FIRST_USER_ID, WPContext::Embed)
        .execute()
        .await
        .unwrap()
        .parse(wp_api::parse_retrieve_user_response_with_embed_context)
        .is_ok());
}

#[tokio::test]
async fn retrieve_user_with_view_context() {
    assert!(api()
        .retrieve_user_request(FIRST_USER_ID, WPContext::View)
        .execute()
        .await
        .unwrap()
        .parse(wp_api::parse_retrieve_user_response_with_view_context)
        .is_ok());
}

#[tokio::test]
async fn retrieve_current_user_with_edit_context() {
    assert!(api()
        .retrieve_current_user_request(WPContext::Edit)
        .execute()
        .await
        .unwrap()
        .parse(wp_api::parse_retrieve_user_response_with_edit_context)
        .is_ok());
}

#[tokio::test]
async fn retrieve_current_user_with_embed_context() {
    assert!(api()
        .retrieve_current_user_request(WPContext::Embed)
        .execute()
        .await
        .unwrap()
        .parse(wp_api::parse_retrieve_user_response_with_embed_context)
        .is_ok());
}

#[tokio::test]
async fn retrieve_current_user_with_view_context() {
    assert!(api()
        .retrieve_current_user_request(WPContext::View)
        .execute()
        .await
        .unwrap()
        .parse(wp_api::parse_retrieve_user_response_with_view_context)
        .is_ok());
}

async fn test_user_list_params(params: UserListParams) {
    let parsed_response = api()
        .list_users_request(WPContext::Edit, &Some(params))
        .execute()
        .await
        .unwrap()
        .parse(wp_api::parse_list_users_response_with_edit_context);
    assert!(
        parsed_response.is_ok(),
        "Response was: '{:?}'",
        parsed_response
    );
}

fn validate_sparse_user_fields(user: &SparseUser, fields: &[SparseUserField]) {
    assert_eq!(user.id.is_some(), fields.contains(&SparseUserField::Id));
    assert_eq!(
        user.username.is_some(),
        fields.contains(&SparseUserField::Username)
    );
    assert_eq!(user.name.is_some(), fields.contains(&SparseUserField::Name));
    assert_eq!(
        user.last_name.is_some(),
        fields.contains(&SparseUserField::LastName)
    );
    assert_eq!(
        user.email.is_some(),
        fields.contains(&SparseUserField::Email)
    );
    assert_eq!(user.url.is_some(), fields.contains(&SparseUserField::Url));
    assert_eq!(
        user.description.is_some(),
        fields.contains(&SparseUserField::Description)
    );
    assert_eq!(user.link.is_some(), fields.contains(&SparseUserField::Link));
    assert_eq!(
        user.locale.is_some(),
        fields.contains(&SparseUserField::Locale)
    );
    assert_eq!(
        user.nickname.is_some(),
        fields.contains(&SparseUserField::Nickname)
    );
    assert_eq!(user.slug.is_some(), fields.contains(&SparseUserField::Slug));
    assert_eq!(
        user.registered_date.is_some(),
        fields.contains(&SparseUserField::RegisteredDate)
    );
    assert_eq!(
        user.roles.is_some(),
        fields.contains(&SparseUserField::Roles)
    );
    assert_eq!(
        user.capabilities.is_some(),
        fields.contains(&SparseUserField::Capabilities)
    );
    assert_eq!(
        user.extra_capabilities.is_some(),
        fields.contains(&SparseUserField::ExtraCapabilities)
    );
    assert_eq!(
        user.avatar_urls.is_some(),
        fields.contains(&SparseUserField::AvatarUrls)
    );
}

#[template]
#[rstest]
#[case(&[SparseUserField::Id])]
#[case(&[SparseUserField::Username])]
#[case(&[SparseUserField::Name])]
#[case(&[SparseUserField::LastName])]
#[case(&[SparseUserField::Email])]
#[case(&[SparseUserField::Url])]
#[case(&[SparseUserField::Description])]
#[case(&[SparseUserField::Link])]
#[case(&[SparseUserField::Locale])]
#[case(&[SparseUserField::Nickname])]
#[case(&[SparseUserField::Slug])]
#[case(&[SparseUserField::RegisteredDate])]
#[case(&[SparseUserField::Roles])]
#[case(&[SparseUserField::Capabilities])]
#[case(&[SparseUserField::ExtraCapabilities])]
#[case(&[SparseUserField::AvatarUrls])]
#[case(&[SparseUserField::Id, SparseUserField::Name])]
#[case(&[SparseUserField::Email, SparseUserField::Nickname])]
fn filter_fields_cases(#[case] fields: &[SparseUserField]) {}
