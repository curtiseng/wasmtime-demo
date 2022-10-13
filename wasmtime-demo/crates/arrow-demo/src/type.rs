/// ```text
/// device module >>> arrow schema
///                                 >>> arrow byte buffer  >>> store or compute
/// bytes >>> device se/des config
/// ```

/// 一次读够 batch_size，不够直接返回