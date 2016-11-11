#ifndef BLUETOOTH_CHARACTERISTIC_H_
#define BLUETOOTH_CHARACTERISTIC_H_

#ifdef __cplusplus
extern "C" {
#endif

#include <jni.h>
#include "bluetooth-service.h"

typedef struct BluetoothCharacteristic BluetoothCharacteristic;

struct BluetoothCharacteristic
{
    jobject characteristic;
    int count;
};

BluetoothCharacteristic* bluetooth_characteristic_create_characteristic (BluetoothService*, int);

const char* bluetooth_characteristic_get_uuid (BluetoothCharacteristic*);
const char** bluetooth_characteristic_get_flags (BluetoothCharacteristic*);
int bluetooth_characteristic_get_flags_size (BluetoothCharacteristic*);
const int* bluetooth_characteristic_get_gatt_descriptors (BluetoothCharacteristic*);
int bluetooth_characteristic_get_gatt_descriptors_size (BluetoothCharacteristic*);
const int* bluetooth_characteristic_get_value (BluetoothCharacteristic*);
const int bluetooth_characteristic_get_value_size (BluetoothCharacteristic*);
int bluetooth_characteristic_read_value (BluetoothCharacteristic*);
int bluetooth_characteristic_write_value (BluetoothCharacteristic*, const int*, int length);
int bluetooth_characteristic_start_notify (BluetoothCharacteristic*);
int bluetooth_characteristic_stop_notify (BluetoothCharacteristic*);
void bluetooth_characteristic_inc_refcount (BluetoothCharacteristic*);
void bluetooth_characteristic_dec_refcount (BluetoothCharacteristic*);
void bluetooth_characteristic_free_characteristic (BluetoothCharacteristic*);

#ifdef __cplusplus
}; /* extern "C" */
#endif

#endif  // BLUETOOTH_CHARACTERISTIC_H_
