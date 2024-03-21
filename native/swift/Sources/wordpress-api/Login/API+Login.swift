import Foundation
import wordpress_api_wrapper

#if canImport(FoundationNetworking)
import FoundationNetworking
#endif

public extension WordPressAPI {
    static func findRestApiEndpointRoot(forSiteUrl url: URL, using session: URLSession) async throws -> URL? {
        let request = WpNetworkRequest(method: .head, url: url)
        let ephemeralClient = WordPressAPI(urlSession: session, baseUrl: url, authenticationStategy: .none)
        let response = try await ephemeralClient.perform(request: request)

        return try wordpress_api_wrapper.getLinkHeader(response: response, name: "https://api.w.org/")?.asUrl()
    }

    func getRestAPICapabilities(forApiRoot url: URL, using session: URLSession) async throws -> WpapiDetails {
        let wpResponse = try await self.perform(request: WpNetworkRequest(method: .get, url: url, headerMap: nil))
        return try wordpress_api_wrapper.parseApiDetailsResponse(response: wpResponse)
    }
}
