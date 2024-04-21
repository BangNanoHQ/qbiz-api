use serde::{Deserialize, Deserializer, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use serde_repr::*;
use std::fmt;

#[derive(Serialize_enum_str, Deserialize_enum_str, PartialEq, Debug, Clone)]
pub enum ResponseCode {
    // Code: SUCCESS Message: Berhasil
    #[serde(rename = "SUCCESS")]
    Success,
    // Code: NOT_PERMISSION Message: bukan izin
    #[serde(rename = "NOT_PERMISSION")]
    NotPermission,
    // Code: UNAUTHORIZED_INVALID_API_SECRET Message: API Secret tidak valid
    #[serde(rename = "UNAUTHORIZED_INVALID_API_SECRET")]
    UnauthorizedInvalidApiSecret,
    // Code: CLIENT_NOT_SUPPORT_MANUAL_DISBURSEMENT Message: Penarikan manual tidak tersedia
    #[serde(rename = "CLIENT_NOT_SUPPORT_MANUAL_DISBURSEMENT")]
    ClientNotSupportManualDisbursement,
    // Code: CLIENT_NO_VALID_BANK_CARD Message: Tidak ada rekening bank yang valid
    #[serde(rename = "CLIENT_NO_VALID_BANK_CARD")]
    ClientNoValidBankCard,
    // Code: CLIENT_INVALID_API_SECRET Message: API Secret tidak valid
    #[serde(rename = "CLIENT_INVALID_API_SECRET")]
    ClientInvalidApiSecret,
    // Code: CLIENT_TOO_FREQUENT Message: Penarikan terlalu sering, silahkan coba lagi nanti
    #[serde(rename = "CLIENT_TOO_FREQUENT")]
    ClientTooFrequent,
    // Code: CLIENT_INVALID_ORDER_STATUS Message: Status permintaan tidak valid
    #[serde(rename = "CLIENT_INVALID_ORDER_STATUS")]
    ClientInvalidOrderStatus,
    // Code: CLIENT_INVALID_LIQUIDATOR Message: invalid liquidator
    #[serde(rename = "CLIENT_INVALID_LIQUIDATOR")]
    ClientInvalidLiquidator,
    // Code: CLIENT_INVALID_SIGN Message: invalid sign
    #[serde(rename = "CLIENT_INVALID_SIGN")]
    ClientInvalidSign,
    // Code: CLIENT_NO_VALID_PLATFORM Message: Tidak ada informasi platform yang valid
    #[serde(rename = "CLIENT_NO_VALID_PLATFORM")]
    ClientNoValidPlatform,
    // Code: CLIENT_QRIS_CREATE_FAILED Message: Gagal membuat QRIS
    #[serde(rename = "CLIENT_QRIS_CREATE_FAILED")]
    ClientQrisCreateFailed,
    // Code: CLIENT_INVALID_PARAMETER Message: Kesalahan parameter
    #[serde(rename = "CLIENT_INVALID_PARAMETER")]
    ClientInvalidParameter,
    // Code: CLIENT_INVALID_ORDER_NO Message: Nomor pesanan tidak valid
    #[serde(rename = "CLIENT_INVALID_ORDER_NO")]
    ClientInvalidOrderNo,
    // Code: CLIENT_INVALID_USERNAME Message: Akun tidak valid
    #[serde(rename = "CLIENT_INVALID_USERNAME")]
    ClientInvalidUsername,
    // Code: CLIENT_INVALID_QRCODE_SN Message: Nomor seri kode QR tidak valid
    #[serde(rename = "CLIENT_INVALID_QRCODE_SN")]
    ClientInvalidQrcodeSn,
    // Code: CLIENT_PHONE_USED Message: Nomor HP sudah digunakan
    #[serde(rename = "CLIENT_PHONE_USED")]
    ClientPhoneUsed,
    // Code: CLIENT_MERCHANT_NAME_USED Message: Nama merchant sudah digunakan
    #[serde(rename = "CLIENT_MERCHANT_NAME_USED")]
    ClientMerchantNameUsed,
    // Code: CLIENT_USER_NAME_USED Message: Username sudah digunakan
    #[serde(rename = "CLIENT_USER_NAME_USED")]
    ClientUserNameUsed,
    // Code: CLIENT_USER_EMAIL_USED Message: Email sudah digunakan
    #[serde(rename = "CLIENT_USER_EMAIL_USED")]
    ClientUserEmailUsed,
    // Code: CLIENT_INVALID_PARAMETER_DETAIL Message: Parameter {0} salah
    #[serde(rename = "CLIENT_INVALID_PARAMETER_DETAIL")]
    ClientInvalidParameterDetail,
    // Code: CLIENT_MERCHANT_RATE_NOT_FOUND Message: MDR belum diatur
    #[serde(rename = "CLIENT_MERCHANT_RATE_NOT_FOUND")]
    ClientMerchantRateNotFound,
    // Code: CLIENT_DATA_OPERATION_ERROR Message: Kegagalan operasi data
    #[serde(rename = "CLIENT_DATA_OPERATION_ERROR")]
    ClientDataOperationError,
    // Code: CLIENT_INVALID_DEVICE_SN Message: Nomor seri perangkat tidak valid
    #[serde(rename = "CLIENT_INVALID_DEVICE_SN")]
    ClientInvalidDeviceSn,
    // Code: CLIENT_INVALID_DEVICE_PRODUCT_TYPE Message: Tipe perangkat tidak valid
    #[serde(rename = "CLIENT_INVALID_DEVICE_PRODUCT_TYPE")]
    ClientInvalidDeviceProductType,
    // Code: CLIENT_QRCODE_ALREADY_EXIST Message: Saat ini hanya 1 kode QRIS yang bisa dibuat
    #[serde(rename = "CLIENT_QRCODE_ALREADY_EXIST")]
    ClientQrcodeAlreadyExist,
    // Code: SERVER_ERROR Message: Server sedang error
    #[serde(rename = "SERVER_ERROR")]
    ServerError,
    // Code: SERVER_INVALID_CONFIG Message: Item konfigurasi tidak valid
    #[serde(rename = "SERVER_INVALID_CONFIG")]
    ServerInvalidConfig,
    // Code: SERVER_INVALID_PLATFORM Message: Platform tidak valid
    #[serde(rename = "SERVER_INVALID_PLATFORM")]
    ServerInvalidPlatform,
    // Code: FORBIDDEN_ERROR Message: Izin tidak memadai
    #[serde(rename = "FORBIDDEN_ERROR")]
    ForbiddenError,
    // Code: ACCOUNT_EXIST Message: Akun sudah ada
    #[serde(rename = "ACCOUNT_EXIST")]
    AccountExist,
    // Code: DEVICE_NAME_EXIST Message: Nama sudah ada
    #[serde(rename = "DEVICE_NAME_EXIST")]
    DeviceNameExist,
    // Code: PAYMENT_ORDER_EXISTS Message: Ada perintah pembayaran
    #[serde(rename = "PAYMENT_ORDER_EXISTS")]
    PaymentOrderExists,
    // Code: ERROR_BANK Message: Bank yang salah
    #[serde(rename = "ERROR_BANK")]
    ErrorBank,
    // Code: AMOUNT_ONLY_SUPPORT_WHOLE_NUMBERS Message: Jumlah hanya dapat dibulatkan ke bilangan bulat terdekat
    #[serde(rename = "AMOUNT_ONLY_SUPPORT_WHOLE_NUMBERS")]
    AmountOnlySupportWholeNumbers,
    // Code: PAYMENT_AMOUNT_NOT_IN_RULE Message: Pembayaran tidak sesuai dengan persyaratan
    #[serde(rename = "PAYMENT_AMOUNT_NOT_IN_RULE")]
    PaymentAmountNotInRule,


    #[serde(other)]
    Other(String),
}