package com.plugin.saf

import android.app.Activity
import android.content.Intent
import android.net.Uri
import androidx.activity.result.ActivityResult
import app.tauri.annotation.ActivityCallback
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSArray

@TauriPlugin
class SafPlugin(private val activity: Activity): Plugin(activity) {
    private val implementation = SafImplementation()

    @Command
    fun requestPermission(invoke: Invoke) {
        val intent = Intent(Intent.ACTION_OPEN_DOCUMENT_TREE).apply {
            flags = Intent.FLAG_GRANT_READ_URI_PERMISSION or
                    Intent.FLAG_GRANT_WRITE_URI_PERMISSION or
                    Intent.FLAG_GRANT_PERSISTABLE_URI_PERMISSION or
                    Intent.FLAG_GRANT_PREFIX_URI_PERMISSION
        }
        startActivityForResult(invoke, intent, "handleRequestPermission")
    }

    @ActivityCallback
    fun handleRequestPermission(invoke: Invoke, result: ActivityResult) {
        if (result.resultCode == Activity.RESULT_OK) {
            val uri = result.data?.data
            if (uri != null) {
                activity.contentResolver.takePersistableUriPermission(
                    uri,
                    Intent.FLAG_GRANT_READ_URI_PERMISSION or Intent.FLAG_GRANT_WRITE_URI_PERMISSION
                )
                val ret = JSObject()
                ret.put("uri", uri.toString())
                invoke.resolve(ret)
            } else {
                invoke.reject("No URI returned")
            }
        } else {
            invoke.reject("Permission denied")
        }
    }

    @Command
    fun checkPermission(invoke: Invoke) {
        val uriString = invoke.getArgs().getString("uri")
        if (uriString == null) {
            invoke.reject("URI is required")
            return
        }

        val uri = Uri.parse(uriString)
        val persistedPermissions = activity.contentResolver.persistedUriPermissions
        var hasPermission = false
        for (permission in persistedPermissions) {
            if (permission.uri == uri && permission.isReadPermission && permission.isWritePermission) {
                hasPermission = true
                break
            }
        }

        val ret = JSObject()
        ret.put("granted", hasPermission)
        invoke.resolve(ret)
    }

    @Command
    fun listDir(invoke: Invoke) {
        val uriString = invoke.getArgs().getString("uri")
        if (uriString == null) {
            invoke.reject("URI is required")
            return
        }

        try {
            val files = implementation.listDir(activity, uriString)
            val ret = JSObject()
            val array = JSArray()
            files.forEach { file ->
                val obj = JSObject()
                obj.put("name", file["name"])
                obj.put("is_directory", file["is_directory"])
                obj.put("uri", file["uri"])
                array.put(obj)
            }
            ret.put("files", array)
            invoke.resolve(ret)
        } catch (e: Exception) {
            invoke.reject(e.message)
        }
    }

    @Command
    fun readFile(invoke: Invoke) {
        val uriString = invoke.getArgs().getString("uri")
        if (uriString == null) {
            invoke.reject("URI is required")
            return
        }

        try {
            val content = implementation.readFile(activity, uriString)
            val ret = JSObject()
            ret.put("content", content)
            invoke.resolve(ret)
        } catch (e: Exception) {
            invoke.reject(e.message)
        }
    }

    @Command
    fun writeFile(invoke: Invoke) {
        val uriString = invoke.getArgs().getString("uri")
        val content = invoke.getArgs().getString("content")
        if (uriString == null || content == null) {
            invoke.reject("URI and content are required")
            return
        }

        try {
            implementation.writeFile(activity, uriString, content)
            invoke.resolve()
        } catch (e: Exception) {
            invoke.reject(e.message)
        }
    }
}