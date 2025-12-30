package com.plugin.saf

import android.content.Context
import android.net.Uri
import androidx.documentfile.provider.DocumentFile
import java.io.OutputStreamWriter
import java.io.BufferedReader
import java.io.InputStreamReader

class SafImplementation {
    fun listDir(context: Context, uriString: String): List<Map<String, Any>> {
        val uri = Uri.parse(uriString)
        val root = DocumentFile.fromTreeUri(context, uri)
        val result = mutableListOf<Map<String, Any>>()
        
        root?.listFiles()?.forEach { file ->
            val map = mutableMapOf<String, Any>()
            map["name"] = file.name ?: ""
            map["is_directory"] = file.isDirectory
            map["uri"] = file.uri.toString()
            result.add(map)
        }
        return result
    }

    fun readFile(context: Context, uriString: String): String {
        val uri = Uri.parse(uriString)
        context.contentResolver.openInputStream(uri).use { inputStream ->
            BufferedReader(InputStreamReader(inputStream)).use { reader ->
                return reader.readText()
            }
        }
    }

    fun writeFile(context: Context, uriString: String, content: String) {
        val uri = Uri.parse(uriString)
        context.contentResolver.openOutputStream(uri, "wt").use { outputStream ->
            OutputStreamWriter(outputStream).use { writer ->
                writer.write(content)
            }
        }
    }
}
