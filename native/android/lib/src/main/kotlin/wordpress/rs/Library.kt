/*
 * This Kotlin source file was generated by the Gradle 'init' task.
 */
package wordpress.rs

import uniffi.wordpress_api.addCustom
import uniffi.wordpress_api.combineStrings
import uniffi.wordpress_api.panicFromRust

class Library {
    fun addCustomFromRust(a: Int, b: Int): Int {
        return addCustom(a, b)
    }

    fun combineStringsFromRust(a: String, b: String): String {
        return combineStrings(a, b)
    }

    fun crashFromRust() {
        return panicFromRust();
    }
}
