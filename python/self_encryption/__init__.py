try:
    from ._rust import (
        DataMap,
        EncryptedChunk,
        StreamSelfEncryptor,
        StreamSelfDecryptor,
        encrypt_bytes,
        decrypt_chunks,
        encrypt_file,
        decrypt_from_files,
        shrink_data_map,
        get_root_data_map,
    )

    __all__ = [
        'DataMap',
        'EncryptedChunk',
        'encrypt_bytes',
        'decrypt_chunks',
        'encrypt_file',
        'decrypt_from_files',
        'shrink_data_map',
        'get_root_data_map',
        'StreamSelfEncryptor',
        'StreamSelfDecryptor'
    ]
except ImportError as e:
    import sys
    print(f"Error importing Rust bindings: {e}", file=sys.stderr)
    raise
