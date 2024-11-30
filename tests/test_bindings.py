import os
import tempfile
from pathlib import Path
from typing import List
import pytest
from self_encryption import (
    DataMap,
    XorName,
    encrypt_from_file,
    decrypt_from_storage,
    streaming_decrypt_from_storage,
)

def test_file_encryption_decryption():
    with tempfile.TemporaryDirectory() as temp_dir:
        # Create test file
        input_path = Path(temp_dir) / "input.dat"
        data = b"x" * 10_000_000
        input_path.write_bytes(data)
        
        # Create output directory for chunks
        chunk_dir = Path(temp_dir) / "chunks"
        chunk_dir.mkdir()
        
        # Encrypt file
        result = encrypt_from_file(str(input_path), str(chunk_dir))
        data_map = result.data_map
        
        # Decrypt to new file
        output_path = Path(temp_dir) / "output.dat"
        decrypt_from_storage(data_map, str(output_path), str(chunk_dir))
        
        # Verify
        assert input_path.read_bytes() == output_path.read_bytes()

def test_streaming_decryption():
    with tempfile.TemporaryDirectory() as temp_dir:
        # Create test file
        input_path = Path(temp_dir) / "input.dat"
        data = b"x" * 10_000_000  # 10MB
        input_path.write_bytes(data)
        
        # Create output directory for chunks
        chunk_dir = Path(temp_dir) / "chunks"
        chunk_dir.mkdir()
        
        # Encrypt file
        result = encrypt_from_file(str(input_path), str(chunk_dir))
        data_map = result.data_map
        
        # Decrypt using streaming
        output_path = Path(temp_dir) / "output.dat"
        streaming_decrypt_from_storage(data_map, str(output_path), str(chunk_dir))
        
        # Verify
        assert input_path.read_bytes() == output_path.read_bytes()

if __name__ == "__main__":
    pytest.main([__file__]) 