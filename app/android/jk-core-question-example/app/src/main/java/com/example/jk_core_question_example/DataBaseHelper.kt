package com.example.jk_core_question_example

import android.content.Context
import android.util.Log
import java.io.*

class DatabaseHelper(context: Context) {
    private val mContext: Context = context.applicationContext
    private val dbName = "car.db"
    private val dbPath = "/data/data/${mContext.packageName}/databases/"

    fun copyDatabaseFromAssets(): String {
        val assetManager = mContext.assets
        val inputStream: InputStream
        val outputStream: OutputStream
        val dbFile = File(dbPath + dbName)

        try {
            if (dbFile.exists()) {
                Log.i("DatabaseHelper", "database file already exist.")
                return dbFile.absolutePath
            }

            val dbDir = dbFile.parentFile
            if (dbDir != null) {
                if (!dbDir.exists()) {
                    dbDir.mkdirs()
                }
            }

            inputStream = assetManager.open(dbName)
            outputStream = FileOutputStream(dbFile)

            val buffer = ByteArray(1024)
            var length: Int
            while (inputStream.read(buffer).also { length = it } > 0) {
                outputStream.write(buffer, 0, length)
            }

            outputStream.flush()
            outputStream.close()
            inputStream.close()

            Log.i("DatabaseHelper", "copy database file success.")
        } catch (e: IOException) {
            Log.e("DatabaseHelper", "Failed to copy database file.", e)
        }

        return dbFile.absolutePath
    }
}
