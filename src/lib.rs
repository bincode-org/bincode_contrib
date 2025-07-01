pub fn decode_whole<D: bincode::de::Decode<()>, C: bincode::config::Config>(
    src: &[u8],
    config: C,
) -> Result<D, bincode::error::DecodeError> {
    let (t, consumed) = bincode::decode_from_slice(src, config)?;

    if consumed != src.len() {
        return Err(bincode::error::DecodeError::Other("leftover bytes"));
    }

    Ok(t)
}
