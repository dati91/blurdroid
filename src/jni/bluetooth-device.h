#ifndef BLUETOOTH_DEVICE_H_
#define BLUETOOTH_DEVICE_H_

#ifdef __cplusplus
extern "C" {
#endif

#include <jni.h>
#include "bluetooth-adapter.h"

typedef struct BluetoothDevice BluetoothDevice;

struct BluetoothDevice
{
    jobject device;
    int count;
};

BluetoothDevice* bluetooth_device_create_device (BluetoothAdapter*, const char*, int);

const char* bluetooth_device_get_address (BluetoothDevice*);
const char* bluetooth_device_get_name (BluetoothDevice*);
const char** bluetooth_device_get_uuids (BluetoothDevice*);
int bluetooth_device_get_uuids_size (BluetoothDevice*);
int bluetooth_device_get_rssi (BluetoothDevice*);
int bluetooth_device_get_tx_power (BluetoothDevice*);
const int* bluetooth_device_get_manufacturer_data_keys (BluetoothDevice*);
int bluetooth_device_get_manufacturer_data_keys_size (BluetoothDevice*);
const int* bluetooth_device_get_manufacturer_data_values (BluetoothDevice*, int);
int bluetooth_device_get_manufacturer_data_values_size (BluetoothDevice*, int);
const char** bluetooth_device_get_service_data_keys (BluetoothDevice*);
int bluetooth_device_get_service_data_keys_size (BluetoothDevice*);
const int* bluetooth_device_get_service_data_values (BluetoothDevice*, const char*, int);
int bluetooth_device_get_service_data_values_size (BluetoothDevice*, const char*, int);
int bluetooth_device_connect_gatt (BluetoothDevice*);
int bluetooth_device_disconnect (BluetoothDevice*);
int bluetooth_device_is_connected (BluetoothDevice*);
const int* bluetooth_device_get_gatt_services (BluetoothDevice*);
int bluetooth_device_get_gatt_services_size (BluetoothDevice*);
void bluetooth_device_inc_refcount (BluetoothDevice*);
void bluetooth_device_dec_refcount (BluetoothDevice*);
void bluetooth_device_free_device (BluetoothDevice*);

#ifdef __cplusplus
}; /* extern "C" */
#endif

#endif  // BLUETOOTH_DEVICE_H_
