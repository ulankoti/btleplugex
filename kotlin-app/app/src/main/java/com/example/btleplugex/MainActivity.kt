package com.example.btleplugex

import android.Manifest
import android.bluetooth.BluetoothAdapter
import android.bluetooth.BluetoothManager
import android.content.BroadcastReceiver
import android.content.Context
import android.content.Intent
import android.content.IntentFilter
import android.content.pm.PackageManager
import android.os.Build
import android.os.Bundle
import android.util.Log
import android.widget.Button
import android.widget.TextView
import android.widget.Toast
import androidx.activity.ComponentActivity
import androidx.activity.result.ActivityResult
import androidx.activity.result.ActivityResultLauncher
import androidx.activity.result.contract.ActivityResultContracts
import androidx.core.content.ContextCompat
import com.example.btleplug

class MainActivity : ComponentActivity(), Runnable {
    private val mTAG = "btleplugex"
    private var isBluetoothON: Boolean = false
    private var isBtOnRequestRejected: Boolean = false
    private var mPermissionRequestLauncher: ActivityResultLauncher<Array<String>>? = null
    private var isBlePermissionGranted: Boolean = false
    private var isBleAdminPermissionGranted: Boolean = false
    private var isBleScanPermissionGranted: Boolean = false
    private var isBleConnectPermissionGranted: Boolean = false
    private var isBleFineLocationPermissionGranted: Boolean = false
    private var isBleCoarseLocationPermissionGranted: Boolean = false
    private lateinit var mScanButton: Button
    private lateinit var mConnectButton: Button
    private lateinit var mServiceCharacteristicsButton: Button
    private lateinit var mSubscribeButton: Button
    private lateinit var btleplug: btleplug
    private lateinit var mActiveButton: Button
    private val maxCounter = 100

    private fun requestPermissions() {
        val permissionRequest: MutableList<String> = ArrayList()
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            isBleScanPermissionGranted = ContextCompat.checkSelfPermission(
                this,
                Manifest.permission.BLUETOOTH_SCAN
            ) == PackageManager.PERMISSION_GRANTED
            if (!isBleScanPermissionGranted) {
                permissionRequest.add(Manifest.permission.BLUETOOTH_SCAN)
            }
            isBleConnectPermissionGranted = ContextCompat.checkSelfPermission(
                this,
                Manifest.permission.BLUETOOTH_CONNECT
            ) == PackageManager.PERMISSION_GRANTED
            if (!isBleConnectPermissionGranted) {
                permissionRequest.add(Manifest.permission.BLUETOOTH_CONNECT)
            }
        } else {
            isBlePermissionGranted = ContextCompat.checkSelfPermission(
                this,
                Manifest.permission.BLUETOOTH
            ) == PackageManager.PERMISSION_GRANTED
            if (!isBlePermissionGranted) {
                permissionRequest.add(Manifest.permission.BLUETOOTH)
            }
            isBleAdminPermissionGranted = ContextCompat.checkSelfPermission(
                this,
                Manifest.permission.BLUETOOTH_ADMIN
            ) == PackageManager.PERMISSION_GRANTED
            if (!isBleAdminPermissionGranted) {
                permissionRequest.add(Manifest.permission.BLUETOOTH_ADMIN)
            }
            isBleFineLocationPermissionGranted = ContextCompat.checkSelfPermission(
                this,
                Manifest.permission.ACCESS_FINE_LOCATION
            ) == PackageManager.PERMISSION_GRANTED
            if (!isBleFineLocationPermissionGranted) {
                permissionRequest.add(Manifest.permission.ACCESS_FINE_LOCATION)
            }
            isBleCoarseLocationPermissionGranted = ContextCompat.checkSelfPermission(
                this,
                Manifest.permission.ACCESS_COARSE_LOCATION
            ) == PackageManager.PERMISSION_GRANTED
            if (!isBleCoarseLocationPermissionGranted) {
                permissionRequest.add(Manifest.permission.ACCESS_COARSE_LOCATION)
            }
        }
        if (permissionRequest.isNotEmpty()) {
            Log.d(
                mTAG, "permissions required: " +
                        permissionRequest.toTypedArray().contentToString()
            )
            mPermissionRequestLauncher!!.launch(permissionRequest.toTypedArray())
        }
    }
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        btleplug = btleplug()
        setContentView(R.layout.layout)

        mScanButton = findViewById(R.id.scanButton)
        mConnectButton = findViewById(R.id.connectButton)
        mServiceCharacteristicsButton = findViewById(R.id.characteristicsButton)
        mSubscribeButton = findViewById(R.id.subscribeButton)

        mScanButton.setOnClickListener {
            mActiveButton = mScanButton
            Thread(this).start()
        }
        mConnectButton.setOnClickListener {
            mActiveButton = mConnectButton
            Thread(this).start()
        }
        mServiceCharacteristicsButton.setOnClickListener {
            mActiveButton = mServiceCharacteristicsButton
            Thread(this).start()
        }
        mSubscribeButton.setOnClickListener {
            mActiveButton = mSubscribeButton
            Thread(this).start()
        }

        if (!packageManager.hasSystemFeature(PackageManager.FEATURE_BLUETOOTH_LE)) {
            Toast.makeText(this, R.string.ble_not_supported, Toast.LENGTH_SHORT).show()
            finish()
        }

        mPermissionRequestLauncher = registerForActivityResult(
            ActivityResultContracts.RequestMultiplePermissions()
        ) { result: Map<String, Boolean> ->
            if (result[Manifest.permission.BLUETOOTH] != null) {
                isBlePermissionGranted = java.lang.Boolean.TRUE == result[Manifest.permission.BLUETOOTH]
            }
            if (result[Manifest.permission.BLUETOOTH_ADMIN] != null) {
                isBleAdminPermissionGranted =
                    java.lang.Boolean.TRUE == result[Manifest.permission.BLUETOOTH_ADMIN]
            }
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
                if (result[Manifest.permission.BLUETOOTH_SCAN] != null) {
                    isBleScanPermissionGranted =
                        java.lang.Boolean.TRUE == result[Manifest.permission.BLUETOOTH_SCAN]
                }
                if (result[Manifest.permission.BLUETOOTH_CONNECT] != null) {
                    isBleConnectPermissionGranted =
                        java.lang.Boolean.TRUE == result[Manifest.permission.BLUETOOTH_CONNECT]
                }
            }
            if (result[Manifest.permission.ACCESS_FINE_LOCATION] != null) {
                isBleFineLocationPermissionGranted =
                    java.lang.Boolean.TRUE == result[Manifest.permission.ACCESS_FINE_LOCATION]
            }
            if (result[Manifest.permission.ACCESS_COARSE_LOCATION] != null) {
                isBleCoarseLocationPermissionGranted =
                    java.lang.Boolean.TRUE == result[Manifest.permission.ACCESS_COARSE_LOCATION]
            }
        }
        requestPermissions()
        registerReceiver(mBCReceiver, IntentFilter(BluetoothAdapter.ACTION_STATE_CHANGED))
    }

    private val mBCReceiver: BroadcastReceiver = object : BroadcastReceiver() {
        override fun onReceive(context: Context, intent: Intent) {
            if (BluetoothAdapter.ACTION_STATE_CHANGED == intent.action) {
                val extra = intent.getIntExtra(
                    BluetoothAdapter.EXTRA_STATE,
                    BluetoothAdapter.STATE_OFF
                )
                if (extra == BluetoothAdapter.STATE_ON) {
                    isBluetoothON = true
                } else if (extra == BluetoothAdapter.STATE_OFF) {
                    isBluetoothON = false
                }
            }
        }
    }

    private val activityResultLauncher = registerForActivityResult(
        ActivityResultContracts.StartActivityForResult()
    ) { result: ActivityResult ->
        if (result.resultCode == RESULT_CANCELED) {
            isBtOnRequestRejected = true
        }
    }

    override fun onStart() {
        super.onStart()
        Log.d(mTAG, "onStart()")
        if (getSystemService(BluetoothManager::class.java).adapter.isEnabled) {
            isBluetoothON = true
        }
    }

    override fun onResume() {
        super.onResume()
        Log.d(mTAG, "onResume()")
        if (!isBluetoothON && !isBtOnRequestRejected) {
            // BT is OFF and BT ON dialog not denied, launch dialog
            if (Build.VERSION.SDK_INT < Build.VERSION_CODES.S || isBleConnectPermissionGranted) {
                val intent = Intent(BluetoothAdapter.ACTION_REQUEST_ENABLE)
                activityResultLauncher.launch(intent)
            }
        }
    }

    override fun onDestroy() {
        super.onDestroy()
        Log.d(mTAG, "onDestroy()")
        unregisterReceiver(mBCReceiver)
    }

    override fun run() {
        val startText: TextView = findViewById(R.id.starttextView)
        val connectText: TextView = findViewById(R.id.connecttextView)
        val servicesText: TextView = findViewById(R.id.characteristicstextView)
        val subscribeText: TextView = findViewById(R.id.subscribetextView)

        when (mActiveButton) {
            mScanButton -> {
                for (i in 1..maxCounter) {
                    Log.d(mTAG, "run() calling btleplug.scanStartStop() method $i time")
                    runOnUiThread {
                        mScanButton.isEnabled = false
                        startText.text = getString(R.string.scan_start_stop_running, i)
                    }
                    btleplug.scanStartStop()
                }
                runOnUiThread {
                    mScanButton.isEnabled = true
                    startText.text = getString(R.string.scan_start_stop_completed)
                }
            }
            mConnectButton -> {
                for (i in 1..maxCounter) {
                    Log.d(mTAG, "run() calling btleplug.connectDisconnect() method $i time")
                    runOnUiThread {
                        mConnectButton.isEnabled = false
                        connectText.text = getString(R.string.connect_disconnect_running, i)
                    }
                    btleplug.connectDisconnect()
                }
                runOnUiThread {
                    mConnectButton.isEnabled = true
                    connectText.text = getString(R.string.connect_disconnect_completed)
                }
            }
            mServiceCharacteristicsButton -> {
                for (i in 1..maxCounter) {
                    Log.d(mTAG, "run() calling btleplug.servicesCharacteristics() method $i time")
                    runOnUiThread {
                        mServiceCharacteristicsButton.isEnabled = false
                        servicesText.text = getString(R.string.services_characteristics_running, i)
                    }
                    btleplug.servicesCharacteristics()
                }
                runOnUiThread {
                    mServiceCharacteristicsButton.isEnabled = true
                    servicesText.text = getString(R.string.services_characteristics_completed)
                }
            }
            mSubscribeButton -> {
                for (i in 1..maxCounter) {
                    Log.d(mTAG, "run() calling btleplug.subscribe() method $i time")
                    runOnUiThread {
                        mSubscribeButton.isEnabled = false
                        subscribeText.text = getString(R.string.subscribe_running, i)
                    }
                    btleplug.subscribe()
                }
                runOnUiThread {
                    mSubscribeButton.isEnabled = true
                    subscribeText.text = getString(R.string.subscribe_completed)
                }
            }
            else -> {
                Log.d(mTAG, "run() calling btleplug.run() method")
                btleplug.run()
            }
        }
    }
}
