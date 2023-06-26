package com.example

import android.util.Log

class btleplug {
    private val TAG = "btleplug"
    external fun run()
    external fun scanStartStop()
    init {
        try {
            System.loadLibrary("btleplugex")
            Log.i(TAG, "loaded libbtleplugex.so")
        } catch (e: Exception) {
            Log.e(TAG, "failed to load libbtleplugex.so: $e")
        }
    }
}
