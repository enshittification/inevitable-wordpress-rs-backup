package rs.wordpress.api.kotlin

import uniffi.wp_api.SparseUser
import uniffi.wp_api.SparseUserField
import uniffi.wp_api.UserCreateParams
import uniffi.wp_api.UserDeleteParams
import uniffi.wp_api.UserDeleteResponse
import uniffi.wp_api.UserId
import uniffi.wp_api.UserListParams
import uniffi.wp_api.UserUpdateParams
import uniffi.wp_api.UserWithEditContext
import uniffi.wp_api.UserWithEmbedContext
import uniffi.wp_api.UserWithViewContext
import uniffi.wp_api.WpContext

interface UsersEndpoint {
    val list: UsersEndpointList
    val retrieve: UsersEndpointRetrieve
    val me: UsersEndpointRetrieveMe
    val create: UsersEndpointCreate
    val update: UsersEndpointUpdate
    val delete: UsersEndpointDelete
}

interface UsersEndpointList {
    suspend fun withEditContext(params: UserListParams?): WpRequestResult<List<UserWithEditContext>>
    suspend fun withEmbedContext(params: UserListParams?): WpRequestResult<List<UserWithEmbedContext>>
    suspend fun withViewContext(params: UserListParams?): WpRequestResult<List<UserWithViewContext>>
    suspend fun filter(
        context: WpContext,
        params: UserListParams?,
        fields: List<SparseUserField>
    ): WpRequestResult<List<SparseUser>>
}

interface UsersEndpointRetrieve {
    suspend fun withEditContext(userId: UserId): WpRequestResult<UserWithEditContext>
    suspend fun withEmbedContext(userId: UserId): WpRequestResult<UserWithEmbedContext>
    suspend fun withViewContext(userId: UserId): WpRequestResult<UserWithViewContext>
    suspend fun filter(
        userId: UserId,
        context: WpContext,
        fields: List<SparseUserField>
    ): WpRequestResult<SparseUser>
}

interface UsersEndpointRetrieveMe {
    suspend fun withEditContext(): WpRequestResult<UserWithEditContext>
    suspend fun withEmbedContext(): WpRequestResult<UserWithEmbedContext>
    suspend fun withViewContext(): WpRequestResult<UserWithViewContext>
    suspend fun filter(
        context: WpContext,
        fields: List<SparseUserField>
    ): WpRequestResult<SparseUser>
}

interface UsersEndpointCreate {
    suspend fun new(params: UserCreateParams): WpRequestResult<UserWithEditContext>
}

interface UsersEndpointUpdate {
    suspend fun withId(userId: UserId, params: UserUpdateParams): WpRequestResult<UserWithEditContext>
    suspend fun me(params: UserUpdateParams): WpRequestResult<UserWithEditContext>
}

interface UsersEndpointDelete {
    suspend fun withId(userId: UserId, params: UserDeleteParams): WpRequestResult<UserDeleteResponse>
    suspend fun me(params: UserDeleteParams): WpRequestResult<UserDeleteResponse>
}
