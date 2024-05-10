/*
 * This Kotlin source file was generated by the Gradle 'init' task.
 */
package rs.wordpress.wp_api

import org.junit.Test
import rs.wordpress.wp_api_kotlin.MyClass
import uniffi.wp_api.PostObject
import uniffi.wp_api.RequestMethod
import uniffi.wp_api.WpApiException
import uniffi.wp_api.WpAuthentication
import uniffi.wp_api.WpRestErrorCode
import uniffi.wp_api.WpRestErrorWrapper
import uniffi.wp_api.wpAuthenticationFromUsernameAndPassword
import kotlin.test.assertEquals
import kotlin.test.assertFailsWith

class LibraryTest {
    private val siteUrl = BuildConfig.TEST_SITE_URL
    private val authentication = wpAuthenticationFromUsernameAndPassword(
        username = BuildConfig.TEST_ADMIN_USERNAME,
        password = BuildConfig.TEST_ADMIN_PASSWORD
    )
    private val library = MyClass(siteUrl, authentication)

    @Test
    fun testBasicPostListRequest() {
        val request = library.postListRequest()
        assertEquals(RequestMethod.GET, request.method)
        assertEquals("$siteUrl/wp-json/wp/v2/posts?context=edit&page=1&per_page=10", request.url)
    }

    @Test
    fun testMakeBasicPostListRequest() {
        val postListResponse = library.makePostListRequest()
        val firstPost: PostObject = postListResponse.postList!!.first()
        assertEquals("Hello world!", firstPost.title?.raw )
    }

    @Test
    fun testPostListRequestForbiddenContext() {
        val unauthenticatedLibrary =
            MyClass(siteUrl, WpAuthentication.AuthorizationHeader("invalid_token"))
        val exception = assertFailsWith<WpApiException.RestException> {
            unauthenticatedLibrary.makePostListRequest()
        }
        assertEquals(401u, exception.statusCode)
        assertEquals(
            WpRestErrorCode.ForbiddenContext,
            (exception.restError as WpRestErrorWrapper.Recognized).v1.code
        )
    }
}
